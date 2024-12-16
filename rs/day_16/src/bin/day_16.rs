use day_16::{read_data, Input};

fn main() {
    let contents = read_data("./data");
    let input = Input::from_data(&contents);

    println!("Part 1: {}", input.part_1());
    println!("Part 2: {}", input.part_2());
}
