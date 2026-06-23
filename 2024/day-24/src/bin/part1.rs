use common::custom_error::Result;
use day_24::part1::process;

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let file = include_str!("../../input.txt");

    let result = process(file)?;

    println!("result: {} ({:?})", result, now.elapsed());
    Ok(())
}

// result: 51410244478064 (2.222791ms) dev
// result: 51410244478064 (236.334µs) release
