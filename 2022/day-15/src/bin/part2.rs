use common::custom_error::Result;
use day_15::part2::process;

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let file = include_str!("../../input.txt");
    let result = process(file, 4000000)?;
    println!("answer: `{}`, time: {:?}", result, now.elapsed());
    Ok(())
}

// answer: `11645454855041`, time: 928.7µs
