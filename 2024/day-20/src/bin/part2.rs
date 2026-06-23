use common::custom_error::Result;
use day_20::part2::process;

fn main() -> Result<()> {
    let file = include_str!("../../input2.txt");
    let result = process(file, 100, 20)?;
    println!("{}", result);
    Ok(())
}
