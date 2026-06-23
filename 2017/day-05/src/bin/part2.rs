use common::custom_error::Result;
use day_05::part2::process;

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let file = include_str!("../../input.txt");
    let result = process(file)?;
    println!("answer: `{}`, time: {:?}", result, now.elapsed());
    Ok(())
}

// answer: `21985262`, time: 839.8261ms
