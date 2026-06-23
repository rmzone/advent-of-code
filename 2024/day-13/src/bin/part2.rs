use common::custom_error::Result;
use day_13::part2::process;

fn main() -> Result<()> {
    let file = include_str!("../../input2.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}

// too high 73892334424690 72587986598368
