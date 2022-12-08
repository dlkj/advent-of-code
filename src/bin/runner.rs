#[allow(clippy::wildcard_imports)]
use advent_of_code_2022::*;

fn main() -> Result<(), anyhow::Error> {
    println!(
        "day01: {}, {}",
        day01::solve_part_a()?,
        day01::solve_part_b()?
    );
    println!(
        "day02: {}, {}",
        day02::solve_part_a()?,
        day02::solve_part_b()?
    );
    println!(
        "day03: {}, {}",
        day03::solve_part_a()?,
        day03::solve_part_b()?
    );
    println!(
        "day04: {}, {}",
        day04::solve_part_a()?,
        day04::solve_part_b()?
    );
    println!(
        "day05: {}, {}",
        day05::solve_part_a()?,
        day05::solve_part_b()?
    );
    println!(
        "day06: {}, {}",
        day06::solve_part_a()?,
        day06::solve_part_b()?
    );
    println!(
        "day07: {}, {}",
        day07::solve_part_a()?,
        day07::solve_part_b()?
    );
    Ok(())
}
