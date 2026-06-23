use crate::parse_input;
use cached::proc_macro::cached;
use common::custom_error::{Error, Result};

pub fn process(input: &str) -> Result<String, Error> {
    let (_, (fragments, towels)) = parse_input(input).expect("Should parse!");

    let mut result = 0;

    for towel in towels.iter() {
        result += build_towel_count(towel, &fragments);
        println!("{} {}\n", towel, &result);
    }

    Ok(result.to_string())
}

#[cached(key = "String", convert = r##"{ format!("{design}") }"##)]
fn build_towel_count(design: &str, patterns: &[&str]) -> usize {
    let mut count = 0;

    for &pattern in patterns {
        if design.starts_with(pattern) {
            let new_design = design.strip_prefix(pattern).unwrap();
            if new_design.is_empty() {
                count += 1;
            }
            count += build_towel_count(new_design, patterns);
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!("16", process(input)?);
        Ok(())
    }
}
