use advent_of_code_2022::day01;

fn main() -> Result<(), anyhow::Error> {
    println!(
        "day01: {}, {}",
        day01::solve_part_a()?,
        day01::solve_part_b()?
    );

    Ok(())
}
