use advent_of_code_2022::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn day01a(c: &mut Criterion) {
    c.bench_function("day01a", |b| b.iter(day01::solve_part_a));
}

fn day01b(c: &mut Criterion) {
    c.bench_function("day01b", |b| b.iter(day01::solve_part_b));
}

criterion_group!(benches, day01a, day01b);
criterion_main!(benches);
