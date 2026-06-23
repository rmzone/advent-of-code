use common::custom_error::Result;

pub fn process(_: &str) -> Result<String> {
    // cannot run the program because it is broken
    // so convert to a program
    // ignore instructions that are causing the machine to never hit `sub h -1`

    let a = 1;
    let mut b = 84;
    let mut c = b;
    let mut h = 0;

    if a != 0 {
        b = b * 100 + 100000;
        c = b + 17000;
    }

    while b - c != 17 {
        let mut d = 2;
        while d < b {
            if b % d == 0 {
                h += 1;
                break;
            }
            d += 1;
        }

        b += 17;
    }

    Ok(h.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
