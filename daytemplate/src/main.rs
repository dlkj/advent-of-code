use anyhow::Ok;

fn main() -> Result<(), anyhow::Error> {
    let input = include_str!("../input.txt");

    println!("{input}");

    Ok(())
}
