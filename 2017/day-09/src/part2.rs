use common::custom_error::Result;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let count = filter_garbage(&input);

    Ok(count.to_string())
}

fn filter_garbage(input: &str) -> i32 {
    let mut processing_garbage = false;
    let mut skip_next = false;
    let mut count = 0;

    for ch in input.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }

        match ch {
            '<' => {
                if processing_garbage {
                    count += 1;
                } else {
                    processing_garbage = true;
                }
            }
            '>' => {
                processing_garbage = false;
            }
            '!' => {
                skip_next = true;
            }
            _ => {
                if processing_garbage {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_filter_garbage() {
        assert_eq!(0, filter_garbage("<>"));
        assert_eq!(17, filter_garbage("<random characters>"));
        assert_eq!(3, filter_garbage("<<<<>"));
        assert_eq!(2, filter_garbage("<{!>}>"));
        assert_eq!(0, filter_garbage("<!!>"));
        assert_eq!(0, filter_garbage("<!!!>>"));
        assert_eq!(10, filter_garbage("<{o\"i!a,<{i<a>"));
    }
}
