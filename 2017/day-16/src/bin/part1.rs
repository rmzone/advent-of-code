use common::custom_error::Result;
use day_16::part1::process;

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let file = include_str!("../../input.txt");
    let result = process(file, 16)?;
    println!("answer: `{}`, time: {:?}", result, now.elapsed());
    Ok(())
}

// answer: `bijankplfgmeodhc`, time: 6.749333ms
