use common::custom_error::Result;
use day_08::part2::process;

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let file = include_str!("../../input.txt");

    let result = day_08::part1::process(file)?;

    println!("result: {} ({:?})", result, now.elapsed());
    Ok(())
}
