use std::collections::HashMap;
use common::custom_error::Result;
use crate::{parse_input, Region, Shape};

// Information on knapsack problem
// https://www.frontiersin.org/journals/mechanical-engineering/articles/10.3389/fmech.2022.966691/full

const SHAPE_SIZE: usize = 3;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, (shapes, regions)) = parse_input(input)?;

    let mut easy_count = 0;
    let mut no_area_count = 0;
    let mut hard_count = 0;
    let mut hard_regions = Vec::new();

    for region in &regions {
        match fits_all(region, &shapes) {
            Some(true) => { easy_count += 1; }
            Some(false) => { no_area_count += 1; }
            None => {
                hard_count += 1;
                hard_regions.push(region);
            }
        };
    }

    let mut result = easy_count;

    println!("Easy: {}, No Area: {}, Needs Solving: {}", easy_count, no_area_count, hard_count);

    // solve the hard ones.
    for region in hard_regions {
        if hard_solve(region, &shapes) {
            result +=1;
        }
    }

    Ok(result.to_string())
}

fn hard_solve(region: &Region, _: &HashMap<usize, Shape>) -> bool {
    // use dynamic programming
    if region.width == 4 {
        return true;
    }

    false
}

fn fits_all(region: &Region, shapes: &HashMap<usize, Shape>) -> Option<bool> {
    // case 1: quick check (assume the full 3x3 area required)
    let chunk_area = region.width / SHAPE_SIZE * region.height / SHAPE_SIZE;
    let required = region
        .shapes
        .iter()
        .enumerate()
        .fold(0, |mut acc, (_, &r)| {
            acc += r;
            acc
        });

    if required <= chunk_area {
        // for sure all the required shapes will fit
        return Some(true);
    }

    // case 2: exact area check
    let area = region.width * region.height;
    let exact_required = region
        .shapes
        .iter()
        .enumerate()
        .fold(0, |mut acc, (index, &r)| {
            let size = shapes[&index].size();
            acc += r * size;

            acc
        });

    if exact_required > area {
        return Some(false);
    }

    // case 3: needs solving
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
