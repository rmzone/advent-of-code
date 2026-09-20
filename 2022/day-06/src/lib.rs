pub mod part1;
pub mod part2;

pub fn has_duplicates(input: &str) -> bool {
    for ch in input.chars() {
        if input.chars().filter(|c| c == &ch).count() > 1 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_duplicates() {
        let input = "mjqj";
        assert_eq!(true, has_duplicates(input));

        let input1 = "mjqa";
        assert_eq!(false, has_duplicates(input1));
    }
}
