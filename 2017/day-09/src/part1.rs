use common::custom_error::Result;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let filtered = remove_garbage(input);
    info!("{}", filtered);

    let mut score = 0;
    let mut total = 0;

    for ch in filtered.chars() {
        if ch == '{' {
            score += 1;
        } else if ch == '}' {
            total += score;
            score -= 1;
        }
    }

    Ok(total.to_string())
}

fn remove_garbage(input: &str) -> String {
    let mut filtered = String::new();
    let mut processing_garbage = false;
    let mut skip_next = false;

    for ch in input.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }

        match ch {
            '<' => {
                processing_garbage = true;
            }
            '>' => {
                processing_garbage = false;
            }
            '!' => {
                skip_next = true;
            }
            _ => {
                if !processing_garbage {
                    filtered.push(ch);
                }
            }
        }
    }

    filtered
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_filter() {
        assert_eq!("{}", remove_garbage("{}"));
    }
}
