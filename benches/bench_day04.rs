use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(benches, bench_part_1, bench_part_2);
criterion_main!(benches);

fn bench_part_1(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day04.txt").unwrap();
    c.bench_function("aoc2021::day04::part1", |b| {
        b.iter(|| black_box(aoc2021::day04::part1(&input)));
    });
}

fn bench_part_2(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day04.txt").unwrap();
    c.bench_function("aoc2021::day04::part2", |b| {
        b.iter(|| black_box(aoc2021::day04::part2(&input)));
    });
}
