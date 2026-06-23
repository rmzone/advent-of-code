use common::custom_error::Result;
use day_22::part1::process;

fn main() -> Result<()> {
    let file = include_str!("../../input.txt");
    let result = process(file, 2000)?;
    println!("{}", result);
    Ok(())
}
