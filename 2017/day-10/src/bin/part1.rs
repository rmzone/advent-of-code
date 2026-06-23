use common::custom_error::Result;
use day_10::part1::process;

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let file = include_str!("../../input.txt");
    let result = process(file, 256)?;
    println!("answer: `{}`, time: {:?}", result, now.elapsed());
    Ok(())
}

// answer: `13760`, time: 148.4µs
