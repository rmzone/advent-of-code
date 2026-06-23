use common::custom_error::Result;
use day_18::part1::process;

fn main() -> Result<()> {
    let file = include_str!("../../input1.txt");
    let result = process(file, 70, 1024)?;
    println!("{}", result);
    Ok(())
}
