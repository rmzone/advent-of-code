use crate::{parse_input, Firewall};
use common::custom_error::Result;
use log::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, rules) = parse_input(input)?;

    info!("{:?}", rules);

    let mut delay = 0;

    // find at what depth we have all non-zero remainders.
    // a remainder of 0 means we were caught
    while rules
        .iter()
        .filter(|&rule| (delay + rule.depth) % (2 * (rule.range - 1)) == 0)
        .count() > 0 {
        delay += 1;
    }

    // check that we are not caught with the calculated delay
    let mut firewall = Firewall::new(rules);
    for _ in 0..delay {
        firewall.move_packet();
    }

    let mut me: i32 = -1;
    let mut caught = 0;
    
    while me < firewall.width as i32 - 1i32 {
        me += 1;
        caught += firewall.check_found(me);
        firewall.move_packet();
    }
    
    if caught > 0 {
        panic!("We were caught!");
    }

    Ok(delay.to_string())
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
        assert_eq!("10", process(input)?);
        Ok(())
    }
}
