use common::custom_error::Result;
use day_20::part1::process;

fn main() -> Result<()> {
    let file = include_str!("../../input1.txt");
    let result = process(file, 100)?;
    println!("{}", result);
    Ok(())
}
