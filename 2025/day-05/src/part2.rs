use tracing::info;
use common::custom_error::Result;
use crate::{process_input, Range};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, (ranges, _)) = process_input(input)?;

    let mut merged_ranges: Vec<Range> = Vec::new();
    for range in &ranges {
        merge_range(*range, &mut merged_ranges);
    }

    let result = merged_ranges.iter()
        .fold(0, |acc, range| acc + (range.end - range.start + 1));

    Ok(result.to_string())
}

fn merge_range(range: Range, merged_ranges: &mut Vec<Range>) {
    let mut done = false;
    let mut range = range;

    info!("merging {:?}", range);

    // special case: merged_ranges is empty
    if merged_ranges.is_empty() {
        info!("adding first {:?}", range);
        merged_ranges.push(range);
        return
    }

    while !done {
         let overlap = merged_ranges
            .iter()
            .find(|r|
                (range.start >= r.start && range.start <= r.end)
                    || (range.end <= r.end && range.end >= r.start)
                    || (range.end >= r.end && range.start <= r.start)
            );

        if let Some(overlap) = overlap {
            // case1: complete overlap inside existing, so it is merged into the current overlap.
            // no need to check with any other ranges
            if overlap.start <= range.start && overlap.end >= range.end {
                info!("complete inside {:?}", overlap);
                done = true;
                continue;
            }

            // case2: complete overlap, covers more than existing
            // need to remove item and try others
            if range.start <= overlap.start && range.end >= overlap.end {
                info!("complete outside {:?}", overlap);
                if let Some(pos) = merged_ranges.iter().position(|x| *x == *overlap) {
                    merged_ranges.remove(pos);
                }
                continue;
            }

            // case3: overlap from left
            if overlap.start <= range.start {
                range.start = overlap.end + 1;
                info!("left {:?}", overlap);
                continue;
            }

            // case4: overlap from right
            if overlap.end >= range.end {
                range.end = overlap.start - 1;
                info!("right {:?}", overlap);
                continue;
            }
        }
        else {
            info!("no overlap");
            done = true;
        }

        info!("adding {:?}", range);
       merged_ranges.push(range);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
