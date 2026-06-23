use common::custom_error::Result;
use day_23::part2::process;

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let file = include_str!("../../input.txt");

    let result = process(file)?;

    println!("result: {} ({:?})", result, now.elapsed());
    Ok(())
}

// result: bg,bu,ce,ga,hw,jw,nf,nt,ox,tj,uu,vk,wp (1.138594583s) dev
// result: bg,bu,ce,ga,hw,jw,nf,nt,ox,tj,uu,vk,wp (93.733167ms) release
