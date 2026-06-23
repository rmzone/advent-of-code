use common::custom_error::Result;
use day_24::part2::process;

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let file = include_str!("../../input.txt");

    let result = process(file)?;

    println!("result: {} ({:?})", result, now.elapsed());
    Ok(())
}

// result: gst,khg,nhn,tvb,vdc,z12,z21,z33 (20.4362ms) dev intel
// result: gst,khg,nhn,tvb,vdc,z12,z21,z33 (21.2256ms) release intel
// result: gst,khg,nhn,tvb,vdc,z12,z21,z33 (2.644625ms) release apple
