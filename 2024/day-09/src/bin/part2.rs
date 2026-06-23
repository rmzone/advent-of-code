use common::custom_error::Result;
use day_09::part2::process;

fn main() -> Result<()> {
    let file = include_str!("../../input2.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}

// fixme: I no longer get the correct result 6413328569890
