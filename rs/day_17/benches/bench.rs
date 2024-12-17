use criterion::Criterion;
use day_17::{read_data, Input, PART_1, PART_2};

mod perf;

fn main() {
    bench().final_summary();
}

fn bench() -> Criterion {
    let mut criterion = Criterion::default()
        .configure_from_args()
        .with_profiler(perf::FlamegraphProfiler::new(100));

    let dir = "../../data";
    let contents = read_data(dir);
    let input = Input::from_data(&contents);

    let mut group = criterion.benchmark_group("Day 17");

    let _parse = group.bench_with_input("parse contents", &contents, |bench, data| {
        bench.iter(|| Input::from_data(data));
    });

    let _part_1 = group.bench_with_input("part 1", &input, |bench, day| {
        bench.iter(|| assert_eq!(PART_1, day.part_1(), "Mismatched results for part 1"));
    });

    let _example = group.bench_with_input(
        "example",
        &Input::from_data(
            "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
        ),
        |bench, day| {
            bench.iter(|| {
                assert_eq!(
                    117_440,
                    day.part_2(),
                    "Mismatched results for part 2 example"
                );
            });
        },
    );

    let _part_2 = group.bench_with_input("part 2", &input, |bench, day| {
        bench.iter(|| assert_eq!(PART_2, day.part_2(), "Mismatched results for part 2"));
    });

    let _total = group.bench_with_input("total", &contents, |bench, data| {
        bench.iter(|| {
            let day = Input::from_data(data);
            assert_eq!(PART_1, day.part_1(), "Mismatched result for part 1");
            assert_eq!(PART_2, day.part_2(), "Mismatched result for part 2");
        });
    });

    group.finish();

    criterion
}
