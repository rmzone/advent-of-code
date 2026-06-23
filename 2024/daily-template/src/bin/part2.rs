use {{crate_name}}::part2::process;
use common::custom_error::Result;

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let file = include_str!("../../input.txt");

    let result = process(file)?;

    println!("result: {} ({:?})", result, now.elapsed());
    Ok(())
}
