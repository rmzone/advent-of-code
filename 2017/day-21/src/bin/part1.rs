use day_21::part1::process;
use common::custom_error::Result;

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let file = include_str!("../../input.txt");
    let result = process(file, 5)?;
    println!("answer: `{}`, time: {:?}", result, now.elapsed());
    Ok(())
}
