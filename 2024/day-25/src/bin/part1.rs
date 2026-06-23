use day_25::part1::process;
use common::custom_error::Result;

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let file = include_str!("../../input.txt");

    let result = process(file)?;

    println!("result: {} ({:?})", result, now.elapsed());
    Ok(())
}

// result: 3146 (16.0091ms) dev intel
// result: 3146 (776.4µs) release intel
// result: 3146 (267.8µs) release with functional intel
// result: 3146 (14.510167ms) dev apple
// result: 3146 (239.583µs) release with functional apple
