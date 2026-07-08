use day_18::part2::process;
use common::custom_error::Result;

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let file = include_str!("../../input.txt");
    let result = process(file)?;
    println!("answer: `{}`, time: {:?}", result, now.elapsed());
    Ok(())
}

// answer: `15240`, time: 102.5904m !!!TOO HIGH!!!

