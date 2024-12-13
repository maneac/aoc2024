use core::fmt::Write;
use core::{fmt::Display, iter::Peekable, str::Chars};
use std::path::PathBuf;
use std::{fs, path::Path};

use aes_gcm::{
    aead::{rand_core::RngCore, Aead, OsRng},
    Aes256Gcm, Key, KeyInit, Nonce,
};
use base64::Engine;
use chrono::{Datelike, Utc};
use clap::{Parser, ValueEnum};
use regex::Regex;
use reqwest::Error;
use sailfish::TemplateOnce;

#[derive(clap::Parser, Debug)]
#[expect(clippy::struct_excessive_bools)]
struct Opts {
    #[arg(
        short,
        long = "day",
        value_name = "DAY",
        value_parser = clap::value_parser!(u32).range(1..=25),
        default_value_t = Utc::now().with_timezone(&chrono::offset::FixedOffset::west_opt(5 * 3600).unwrap()).day().min(25),
        help = "Day to download the instructions and input for (defaults to the min(current day, 25) in EST)"
    )]
    day: u32,

    #[arg(long, default_value_t = Utc::now().year() as u32, help = "Competition year")]
    year: u32,

    #[arg(
        short = 'f',
        long = "download",
        help = "Force the download of the instructions"
    )]
    force_download: bool,

    #[arg(long, group = "data", help = "Skip the downloading of the input data")]
    no_data: bool,

    #[arg(long, group = "data", help = "Only decrypt the input data")]
    decrypt_data: bool,

    #[arg(
        short,
        long,
        use_value_delimiter = true,
        value_parser,
        default_value = "go,ts,rs",
        help = "Languages to create instructions and templates for"
    )]
    langs: Vec<Lang>,

    #[arg(long, help = "Skip code template creation for each language")]
    skip_templates: bool,

    #[arg(long, help = "Keep the raw instruction HTML file")]
    keep_instructions: bool,

    #[arg(
        long,
        help = "Update the READMEs to contain part 2. Alias for '--download --no-data --skip-templates'"
    )]
    part_2: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Lang {
    Go,
    Rs,
    Ts,
}

impl Display for Lang {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::Go => formatter.write_str("go"),
            Self::Rs => formatter.write_str("rs"),
            Self::Ts => formatter.write_str("ts"),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let opts = {
        let mut opts = Opts::parse();
        if opts.part_2 {
            opts.force_download = true;
            opts.no_data = true;
            opts.skip_templates = true;
        }
        opts
    };

    let _loaded_env = dotenvy::dotenv().ok();

    let Ok(aoc_token) = std::env::var("AOC_SESSION_TOKEN") else {
        anyhow::bail!("No 'AOC_SESSION_TOKEN' set");
    };
    let Ok(key) = std::env::var("AOC_AES_KEY") else {
        anyhow::bail!("No 'AOC_AES_KEY' set");
    };
    let day_url = format!("https://adventofcode.com/{}/day/{}", opts.year, opts.day);

    if opts.decrypt_data {
        return decrypt_data(key.as_bytes());
    }

    let day = DayBuilder::new(opts, aoc_token, &key, day_url);

    day.write_data_file()?;
    day.write_instruction_files()?;

    for &lang in &day.opts.langs {
        match lang {
            Lang::Ts => day.add_ts_template()?,
            Lang::Go => day.add_go_template()?,
            Lang::Rs => day.add_rs_template()?,
        }
    }

    Ok(())
}

struct DayBuilder {
    opts: Opts,
    aoc_token: String,
    aoc_aes_key: Aes256Gcm,
    base_url: String,
    package_name: String,
    display_name: String,
}

impl DayBuilder {
    fn new(opts: Opts, aoc_token: String, data_key: &str, base_url: String) -> Self {
        let key = Key::<Aes256Gcm>::from_slice(data_key.as_bytes());
        let aoc_aes_key = Aes256Gcm::new(key);

        let package_name = format!("day_{:02}", opts.day);
        let display_name = format!("Day {:02}", opts.day);

        Self {
            opts,
            aoc_token,
            aoc_aes_key,
            base_url,
            package_name,
            display_name,
        }
    }

    fn write_data_file(&self) -> anyhow::Result<()> {
        if self.opts.no_data {
            return Ok(());
        }

        let data_dir = Path::new("data");
        fs::create_dir_all(data_dir)?;

        let data_file = data_dir.join(format!("{}.txt", self.package_name));
        if data_file.exists() {
            return Ok(());
        }

        let data = self.get_from_aoc(Some("input"))?;
        fs::write(data_file, &data)?;

        let mut nonce = [0_u8; 12];
        OsRng.fill_bytes(&mut nonce);

        let nonce = Nonce::from(nonce);

        let ciphertext = self.aoc_aes_key.encrypt(&nonce, data.as_bytes())?;

        let encoded = base64::engine::general_purpose::STANDARD.encode(
            nonce
                .iter()
                .chain(ciphertext.iter())
                .copied()
                .collect::<Vec<u8>>(),
        );

        fs::write(
            data_dir.join(format!("{}.enc.txt", self.package_name)),
            encoded,
        )?;

        Ok(())
    }

    fn write_instruction_files(&self) -> anyhow::Result<()> {
        let instruction_file = Path::new("instructions.html");

        let instructions_html = if !self.opts.force_download && instruction_file.exists() {
            let instructions = fs::read_to_string(instruction_file)?;
            if !self.opts.keep_instructions {
                fs::remove_file(instruction_file)?;
            }
            instructions
        } else {
            let instructions = self.get_from_aoc(None)?;
            if self.opts.keep_instructions {
                fs::write(instruction_file, &instructions)?;
            }
            instructions
        };

        let Some(parts) = Regex::new("(?s)<article.*?>(.+?)</article>")?
            .captures_iter(&instructions_html)
            .map(|caps| caps.get(1).map(|capture| capture.as_str()))
            .collect::<Option<Vec<&str>>>()
        else {
            anyhow::bail!("Failed to extract parts from instructions page")
        };

        let mut readme_contents = String::new();
        for part in parts {
            let mut iter = part.chars().peekable();
            while iter.peek().is_some() {
                let out = recursive_parse_instructions(&self.base_url, &mut iter, false)?;
                readme_contents.push_str(&out);
            }
            readme_contents.push('\n');
        }

        for lang in &self.opts.langs {
            let dir = Path::new(&lang.to_string()).join(&self.package_name);
            fs::create_dir_all(&dir)?;
            fs::write(dir.join("README.md"), readme_contents.as_str())?;
        }

        Ok(())
    }

    fn add_go_template(&self) -> anyhow::Result<()> {
        if self.opts.skip_templates {
            return Ok(());
        }

        let dir = Path::new("go").join(&self.package_name);

        MainGo {
            package_name: &self.package_name,
        }
        .write(&dir)?;

        MainTestGo {}.write(&dir)?;

        Ok(())
    }

    fn add_ts_template(&self) -> anyhow::Result<()> {
        if self.opts.skip_templates {
            return Ok(());
        }

        let dir = Path::new("ts").join(&self.package_name);

        MainTs {}.write(&dir)?;

        DayTs {
            package_name: &self.package_name,
        }
        .write(&dir)?;

        DayTestTs {}.write(&dir)?;

        DayBenchTs {}.write(&dir)?;

        Ok(())
    }

    fn add_rs_template(&self) -> anyhow::Result<()> {
        if self.opts.skip_templates {
            return Ok(());
        }

        let dir = Path::new("rs").join(&self.package_name);

        fs::create_dir_all(dir.join("src").join("bin"))?;
        fs::create_dir_all(dir.join("benches"))?;

        CargoRs {
            crate_name: &self.package_name,
        }
        .write(&dir)?;

        LibRs {
            crate_name: &self.package_name,
        }
        .write(&dir)?;

        MainRs {
            crate_name: &self.package_name,
        }
        .write(&dir)?;

        BenchRs {
            crate_name: &self.package_name,
            display_name: &self.display_name,
        }
        .write(&dir)?;

        Ok(())
    }

    fn get_from_aoc(&self, path: Option<&str>) -> Result<String, Error> {
        let url = path.map_or_else(
            || self.base_url.clone(),
            |url_path| format!("{}/{}", self.base_url, url_path),
        );

        let client = reqwest::blocking::Client::new();
        let req = client
            .get(url)
            .header("Cookie", format!("session={}", self.aoc_token))
            .build()?;

        client.execute(req)?.text()
    }
}

trait Template {
    fn write(self, base: impl AsRef<Path>) -> anyhow::Result<()>;
}

#[derive(TemplateOnce)]
#[template(path = "rs/src/lib.stpl.rs")]
struct LibRs<'a> {
    crate_name: &'a str,
}

impl Template for LibRs<'_> {
    fn write(self, base: impl AsRef<Path>) -> anyhow::Result<()> {
        fs::write(base.as_ref().join("src").join("lib.rs"), {
            let mut output = self.render_once()?;
            output.push('\n');
            output
        })?;
        Ok(())
    }
}

#[derive(TemplateOnce)]
#[template(path = "rs/Cargo.stpl.toml")]
struct CargoRs<'a> {
    crate_name: &'a str,
}

impl Template for CargoRs<'_> {
    fn write(self, base: impl AsRef<Path>) -> anyhow::Result<()> {
        fs::write(base.as_ref().join("Cargo.toml"), {
            let mut output = self.render_once()?;
            output.push('\n');
            output
        })?;
        Ok(())
    }
}

#[derive(TemplateOnce)]
#[template(path = "rs/src/bin/main.stpl.rs")]
struct MainRs<'a> {
    crate_name: &'a str,
}

impl Template for MainRs<'_> {
    fn write(self, base: impl AsRef<Path>) -> anyhow::Result<()> {
        let Some(day) = base.as_ref().file_name().and_then(|fname| fname.to_str()) else {
            anyhow::bail!(
                "Invalid base path for 'main.rs' generation: {:?}",
                base.as_ref()
            );
        };

        fs::write(
            base.as_ref()
                .join("src")
                .join("bin")
                .join(format!("{day}.rs")),
            {
                let mut output = self.render_once()?;
                output.push('\n');
                output
            },
        )?;
        Ok(())
    }
}

#[derive(TemplateOnce)]
#[template(path = "rs/benches/bench.stpl.rs")]
struct BenchRs<'a> {
    crate_name: &'a str,
    display_name: &'a str,
}

impl Template for BenchRs<'_> {
    fn write(self, base: impl AsRef<Path>) -> anyhow::Result<()> {
        fs::write(base.as_ref().join("benches").join("bench.rs"), {
            let mut output = self.render_once()?;
            output.push('\n');
            output
        })?;
        Ok(())
    }
}

#[derive(TemplateOnce)]
#[template(path = "go/main.stpl.go")]
struct MainGo<'a> {
    package_name: &'a str,
}

impl Template for MainGo<'_> {
    fn write(self, base: impl AsRef<Path>) -> anyhow::Result<()> {
        fs::write(base.as_ref().join("main.go"), {
            let mut output = self.render_once()?;
            output.push('\n');
            output
        })?;
        Ok(())
    }
}

#[derive(TemplateOnce)]
#[template(path = "go/main_test.stpl.go")]
struct MainTestGo {}

impl Template for MainTestGo {
    fn write(self, base: impl AsRef<Path>) -> anyhow::Result<()> {
        fs::write(base.as_ref().join("main_test.go"), {
            let mut output = self.render_once()?;
            output.push('\n');
            output
        })?;
        Ok(())
    }
}

#[derive(TemplateOnce)]
#[template(path = "ts/main.stpl.ts")]
struct MainTs {}

impl Template for MainTs {
    fn write(self, base: impl AsRef<Path>) -> anyhow::Result<()> {
        fs::write(base.as_ref().join("main.ts"), {
            let mut output = self.render_once()?;
            output.push('\n');
            output
        })?;
        Ok(())
    }
}

#[derive(TemplateOnce)]
#[template(path = "ts/day.stpl.ts")]
struct DayTs<'a> {
    package_name: &'a str,
}

impl Template for DayTs<'_> {
    fn write(self, base: impl AsRef<Path>) -> anyhow::Result<()> {
        fs::write(base.as_ref().join("day.ts"), {
            let mut output = self.render_once()?;
            output.push('\n');
            output
        })?;
        Ok(())
    }
}

#[derive(TemplateOnce)]
#[template(path = "ts/day.bench.stpl.ts")]
struct DayBenchTs {}

impl Template for DayBenchTs {
    fn write(self, base: impl AsRef<Path>) -> anyhow::Result<()> {
        fs::write(base.as_ref().join("day.bench.ts"), {
            let mut output = self.render_once()?;
            output.push('\n');
            output
        })?;
        Ok(())
    }
}

#[derive(TemplateOnce)]
#[template(path = "ts/day.test.stpl.ts")]
struct DayTestTs {}

impl Template for DayTestTs {
    fn write(self, base: impl AsRef<Path>) -> anyhow::Result<()> {
        fs::write(base.as_ref().join("day.test.ts"), {
            let mut output = self.render_once()?;
            output.push('\n');
            output
        })?;
        Ok(())
    }
}

fn recursive_parse_instructions<'a>(
    url: &'a str,
    input: &mut Peekable<Chars<'a>>,
    is_preformatted: bool,
) -> anyhow::Result<String> {
    let whole_tag = input.take_while(|char| char.ne(&'>')).collect::<String>();

    let raw_tag = whole_tag
        .chars()
        .take_while(|&char| char.ne(&' '))
        .collect::<String>();

    let mut tag = raw_tag.trim();
    if let Some(stripped_tag) = tag.strip_prefix('<') {
        tag = stripped_tag;
    }

    let mut pre = is_preformatted;
    let mut output = String::new();

    match tag {
        "h2" => output.push_str("\n## "),
        "em" if is_preformatted => output.push_str("<b>"),
        "code" if is_preformatted => output.push_str("<code>"),
        "em" => output.push_str("**"),
        "code" => output.push('`'),
        "pre" => {
            output.push_str("\n\n<pre>");
            pre = true;
        }
        "p" => output.push_str("\n\n"),
        "ul" => output.push('\n'),
        "li" => output.push_str("  - "),
        "a" => output.push('['),
        "span" => {}
        "" if input.peek().is_none() => {}
        _ => {
            anyhow::bail!("unknown tag: {}", tag)
        }
    }

    loop {
        match match input.next() {
            Some(char) => char,
            None => return Ok(output),
        } {
            '<' => {
                if input.peek() == Some(&'/') {
                    let _taken = input.take_while(|char| char.ne(&'>')).collect::<String>();
                    break;
                }
                output.push_str(&recursive_parse_instructions(url, input, pre)?);
            }
            '>' => break,
            char => {
                output.push(char);
            }
        }
    }

    match tag {
        "h2" if output.contains(" --- Day") => {
            output = output.replace("\n## ", "# [");
            write!(&mut output, "]({url})")?;
        }
        "em" if is_preformatted => output.push_str("</b>"),
        "code" if is_preformatted => output.push_str("</code>"),
        "em" => output.push_str("**"),
        "code" => output.push('`'),
        "pre" => output.push_str("</pre>"),
        "a" => {
            let Some(link) = Regex::new(r#"href="(.+?)""#)?
                .captures(&whole_tag)
                .and_then(|caps| caps.get(1).map(|cap| cap.as_str()))
            else {
                anyhow::bail!("Failed to find link following anchor")
            };
            write!(&mut output, "]({link})")?;
        }
        "p" | "span" | "h2" | "ul" | "li" => {}
        _ => {
            anyhow::bail!("unknown tag: {}", tag)
        }
    }

    let mut output = output.trim_end().to_owned();

    // hack to ensure emphasised code blocks have the correct operation order
    if output.starts_with('`') && output.contains("**") {
        output = format!("**{}**", &output.replace("**", ""));
    }

    if !is_preformatted {
        output = output.replace("&gt;", ">");
        output = output.replace("&lt;", "<");
    }

    Ok(output)
}

fn decrypt_data(key: &[u8]) -> anyhow::Result<()> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    let dir = PathBuf::from("./data");
    let dir_iter = fs::read_dir(&dir)?;
    for file in dir_iter {
        let file = file?;
        let file_name = file.file_name();
        let Some(file_name) = file_name.to_str() else {
            anyhow::bail!("Invalid file name for decryption: {:?}", file.path());
        };
        if !file_name.ends_with(".enc.txt") {
            continue;
        }

        let contents = fs::read_to_string(file.path())?;

        let decoded = base64::engine::general_purpose::STANDARD.decode(contents)?;

        let (nonce, ciphertext) = decoded.split_at(12);
        let nonce = nonce.iter().copied().collect::<Nonce<_>>();

        let decrypted = cipher.decrypt(&nonce, ciphertext)?;

        fs::write(dir.join(file_name.replace(".enc.txt", ".txt")), decrypted)?;
    }

    Ok(())
}
