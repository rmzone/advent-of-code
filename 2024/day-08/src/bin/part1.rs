use common::custom_error::Result;
use day_08::part1::process;

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let file = include_str!("../../input.txt");

    let result = process(file)?;

    println!("result: {} ({:?})", result, now.elapsed());
    Ok(())
}

// result: 280 (2.293167ms) dev
