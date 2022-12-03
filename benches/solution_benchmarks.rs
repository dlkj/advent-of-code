use advent_of_code_2022::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn day01a(c: &mut Criterion) {
    c.bench_function("day01a", |b| b.iter(day01::solve_part_a));
}

fn day01b(c: &mut Criterion) {
    c.bench_function("day01b", |b| b.iter(day01::solve_part_b));
}

fn day02a(c: &mut Criterion) {
    c.bench_function("day02a", |b| b.iter(day02::solve_part_a));
}

fn day02b(c: &mut Criterion) {
    c.bench_function("day02b", |b| b.iter(day02::solve_part_b));
}
fn day03a(c: &mut Criterion) {
    c.bench_function("day03a", |b| b.iter(day03::solve_part_a));
}

fn day03b(c: &mut Criterion) {
    c.bench_function("day03b", |b| b.iter(day03::solve_part_b));
}

criterion_group!(benches, day01a, day01b, day02a, day02b, day03a, day03b);
criterion_main!(benches);
