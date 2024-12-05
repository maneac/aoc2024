use criterion::Criterion;
use day_02::{read_data, Input, PART_1, PART_2};

fn main() {
    bench();

    Criterion::default().configure_from_args().final_summary();
}

fn bench() {
    let mut criterion = Criterion::default().configure_from_args();

    let dir = "../../data";
    let contents = read_data(dir);
    let input = Input::from_data(&contents);

    let mut group = criterion.benchmark_group("Day 02");

    group.bench_with_input("parse contents", &contents, |b, i| {
        b.iter(|| Input::from_data(i))
    });

    group.bench_with_input("part 1", &input, |b, i| {
        b.iter(|| assert_eq!(PART_1, i.part_1()))
    });

    group.bench_with_input("part 2", &input, |b, i| {
        b.iter(|| assert_eq!(PART_2, i.part_2()))
    });

    group.bench_with_input("total", &contents, |b, i| {
        b.iter(|| {
            let data = Input::from_data(i);
            assert_eq!(PART_1, data.part_1());
            assert_eq!(PART_2, data.part_2());
        })
    });

    group.finish()
}
