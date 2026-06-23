use common::custom_error::Result;
use day_10::part2::process;

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let file = include_str!("../../input.txt");
    let result = process(file, 256)?;
    println!("answer: `{}`, time: {:?}", result, now.elapsed());
    Ok(())
}

// answer: `2da93395f1a6bb3472203252e3b17fe5`, time: 4.8885ms
