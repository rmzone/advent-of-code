use crate::parse_input;
use common::custom_error::Result;
use log::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, rules) = parse_input(input)?;

    info!("{:?}", rules);

    // using Chinese Remainder Theorem
    // essentially we have triangle waves with period 2 * (rule.range - 1)
    // if rule.depth % (2 * (rule.range - 1)) == 0 means we are caught
    let result: i32 = rules
        .iter()
        .map(|rule| {
            if rule.depth % (2 * (rule.range - 1)) == 0 {
                rule.depth * rule.range
            } else {
                0
            }
        })
        .sum();

    // brute force solution:
    // let mut firewall = Firewall::new(rules);
    //
    // let mut me: i32 = -1;
    // // println!("Initial state:");
    // // firewall.display(me);
    //
    // let mut result = 0;
    //
    // while me < firewall.width as i32 - 1i32 {
    //     me += 1;
    //    // println!("Picosecond: {}", me);
    //     //firewall.display(me);
    //     result += firewall.check_found(me);
    //     firewall.move_packet();
    //    // firewall.display(me);
    // }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "0: 3
1: 2
4: 4
6: 4";
        assert_eq!("24", process(input)?);
        Ok(())
    }
}
