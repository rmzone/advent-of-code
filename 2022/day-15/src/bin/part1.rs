use common::custom_error::Result;
use day_15::part1::process;

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let file = include_str!("../../input.txt");
    let result = process(file, 2000000)?;
    println!("answer: `{}`, time: {:?}", result, now.elapsed());
    Ok(())
}

// answer: `4876693`, time: 8.4354713s
