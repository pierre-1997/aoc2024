use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_part_1(c: &mut Criterion) {
    let input = include_str!("../inputs/part-1.txt");

    c.bench_function("{{crate_name}}::part_1", |b| {
        b.iter(|| {{crate_name}}::part1::process(black_box(input)))
    });
}

pub fn bench_part_2(c: &mut Criterion) {
    let input = include_str!("../inputs/part-2.txt");

    c.bench_function("{{crate_name}}::part_2", |b| {
        b.iter(|| {{crate_name}}::part2::process(black_box(input)))
    });
}

criterion_group!(part1, bench_part_1);
criterion_group!(part2, bench_part_2);
criterion_main!(part1, part2);
