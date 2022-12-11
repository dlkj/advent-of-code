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

fn day04a(c: &mut Criterion) {
    c.bench_function("day04a", |b| b.iter(day04::solve_part_a));
}

fn day04b(c: &mut Criterion) {
    c.bench_function("day04b", |b| b.iter(day04::solve_part_b));
}

fn day05a(c: &mut Criterion) {
    c.bench_function("day05a", |b| b.iter(day05::solve_part_a));
}

fn day05b(c: &mut Criterion) {
    c.bench_function("day05b", |b| b.iter(day05::solve_part_b));
}

fn day06a(c: &mut Criterion) {
    c.bench_function("day06a", |b| b.iter(day06::solve_part_a));
}

fn day06b(c: &mut Criterion) {
    c.bench_function("day06b", |b| b.iter(day06::solve_part_b));
}

fn day07a(c: &mut Criterion) {
    c.bench_function("day07a", |b| b.iter(day07::solve_part_a));
}

fn day07b(c: &mut Criterion) {
    c.bench_function("day07b", |b| b.iter(day07::solve_part_b));
}

fn day08a(c: &mut Criterion) {
    c.bench_function("day08a", |b| b.iter(day08::solve_part_a));
}

fn day08b(c: &mut Criterion) {
    c.bench_function("day08b", |b| b.iter(day08::solve_part_b));
}

fn day09a(c: &mut Criterion) {
    c.bench_function("day09a", |b| b.iter(day09::solve_part_a));
}

fn day09b(c: &mut Criterion) {
    c.bench_function("day09b", |b| b.iter(day09::solve_part_b));
}

fn day10a(c: &mut Criterion) {
    c.bench_function("day10a", |b| b.iter(day10::solve_part_a));
}

fn day10b(c: &mut Criterion) {
    c.bench_function("day10b", |b| b.iter(day10::solve_part_b));
}

fn day11a(c: &mut Criterion) {
    c.bench_function("day11a", |b| b.iter(day11::solve_part_a));
}

fn day11b(c: &mut Criterion) {
    c.bench_function("day11b", |b| b.iter(day11::solve_part_b));
}

criterion_group!(
    benches, day01a, day01b, day02a, day02b, day03a, day03b, day04a, day04b, day05a, day05b,
    day06a, day06b, day07a, day07b, day08a, day08b, day09a, day09b, day10a, day10b, day11a, day11b
);
criterion_main!(benches);
