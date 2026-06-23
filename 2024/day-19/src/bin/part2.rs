use common::custom_error::Result;
use day_19::part2::process;

fn main() -> Result<()> {
    let file = include_str!("../../input2.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}

// too low 2569050233
// actual 632423618484345
