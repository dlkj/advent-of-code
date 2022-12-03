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

    Ok(())
}
