use criterion::Criterion;
use day_06::{read_data, Input, PART_1, PART_2};

fn main() {
    bench();

    Criterion::default().configure_from_args().final_summary();
}

fn bench() {
    let mut criterion = Criterion::default().configure_from_args();

    let dir = "../../data";
    let contents = read_data(dir);
    let input = Input::from_data(&contents);

    let mut group = criterion.benchmark_group("Day 06");

    let _parse = group.bench_with_input("parse contents", &contents, |bench, data| {
        bench.iter(|| Input::from_data(data));
    });

    let _part_1 = group.bench_with_input("part 1", &input, |bench, day| {
        bench.iter(|| assert_eq!(PART_1, day.part_1(), "Mismatched results for part 1"));
    });

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
}
