use common::custom_error::Result;
use day_23::part1::process;

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let file = include_str!("../../input.txt");

    let result = process(file)?;

    println!("result: {} ({:?})", result, now.elapsed());
    Ok(())
}

// result: 1314 (1.244519833s) dev
// result: 1314 (196.817791ms) release
