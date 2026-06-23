use common::custom_error::Result;
use day_14::part2::process;

fn main() -> Result<()> {
    let file = include_str!("../../input2.txt");
    let result = process(file, 101, 103)?;
    println!("{}", result);
    Ok(())
}
