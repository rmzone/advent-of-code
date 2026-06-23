use std::collections::HashMap;
use glam::{IVec2, IVec3, Vec3Swizzles};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending},
    multi::separated_list1,
    IResult,
};
use nom::character::complete;
use nom::sequence::separated_pair;
use common::custom_error::AocError;

#[derive(Debug)]
struct Brick {
    cubes: Vec<IVec3>
}

fn brick_end(input: &str) -> IResult<&str, IVec3> {
    let (input, x) = complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = complete::i32(input)?;

    Ok((input, IVec3::new(x, y, z)))
}

fn brick(input: &str) -> IResult<&str, Brick> {
    let (input, (start, end)) =
        separated_pair(brick_end, tag("~"), brick_end)(input)?;

    let cubes = [start.x..=end.x, start.y..=end.y, start.z..=end.z]
        .into_iter()
        .multi_cartesian_product()
        .map(|coord| {
            IVec3::new(coord[0], coord[1], coord[2])
        })
        .collect();

    Ok((input, Brick { cubes }))
}

fn parse_bricks(input: &str) -> IResult<&str, Vec<Brick>> {
    separated_list1(line_ending, brick)(input)
}

// compare the lowest z-coordinate cube from each brick
fn sort_bricks(bricks: Vec<Brick>) -> Vec<Brick> {
    bricks
        .into_iter()
        .sorted_by(|a, b| {
            a.cubes
                .iter()
                .map(|cube| cube.z)
                .min()
                .unwrap()
                .cmp(
                    &b.cubes
                        .iter()
                        .map(|cube| cube.z)
                        .min()
                        .unwrap(),
                )
        }).collect()
}

fn drop_bricks(bricks: Vec<Brick>) -> Vec<Brick> {
    bricks
        .into_iter().fold(vec![], |mut acc: Vec<Brick>, brick| {
        let min_cubes = brick
            .cubes
            .iter()
            .min_set_by_key(|cube| cube.z);

        let min_cubes_xy: Vec<IVec2> = min_cubes
            .iter()
            .map(|cube| cube.xy())
            .collect();

        let max_z_underneath = acc
            .iter()
            .flat_map(|brick| brick.cubes.iter())
            .filter_map(|cube| {
                min_cubes_xy
                    .contains(&cube.xy())
                    .then_some(cube.z)
            })
            .max()
            .unwrap_or(0);

        let landing_z = max_z_underneath + 1;

        let brick_z =
            min_cubes.iter().next().unwrap().z;

        let diff = brick_z - landing_z;

        let new_cubes = brick
            .cubes
            .iter()
            .map(|cube| {
                IVec3::new(
                    cube.x,
                    cube.y,
                    cube.z - diff,
                )
            })
            .collect();
        acc.push(Brick { cubes: new_cubes });

        acc
    }
    )
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, bricks) = parse_bricks(input).expect("should parse");
    let sorted_bricks = sort_bricks(bricks);
    let fallen_bricks = drop_bricks(sorted_bricks);

    let cube_to_id_map = fallen_bricks
        .iter()
        .enumerate()
        .flat_map(|(id, brick)| {
            brick
                .cubes
                .iter()
                .map(move |cube| (cube.xyz(), id))
        })
        .collect::<HashMap<IVec3, usize>>();

    let id_to_cube_map = cube_to_id_map.iter().fold(
        HashMap::<usize, Vec<&IVec3>>::new(),
        |mut map, (cube, id)| {
            map.entry(*id)
                .and_modify(|arr| {
                    arr.push(cube);
                })
                .or_insert(vec![cube]);
            map
        },
    );

    let dissovable = fallen_bricks
        .iter()
        .filter(|brick| {
            let max_cubes = brick
                .cubes
                .iter()
                .max_set_by_key(|cube| cube.z);
            let our_id = cube_to_id_map[&max_cubes[0]];
            let max_z = max_cubes[0].z;
            // vec of ids
            let bricks_we_support: Vec<usize> = max_cubes
                .iter()
                .filter_map(|cube| {
                    cube_to_id_map.get(&IVec3::new(
                        cube.x,
                        cube.y,
                        max_z + 1,
                    ))
                })
                .unique()
                .cloned()
                .collect();

            if bricks_we_support.is_empty() {
                return true;
            }

            bricks_we_support
                .iter()
                .filter(|brick_id| {
                    let cubes = id_to_cube_map
                        .get(&brick_id)
                        .unwrap();
                    let min_cubes = cubes
                        .iter()
                        .min_set_by_key(|cube| cube.z);

                    min_cubes
                        .iter()
                        .filter_map(|cube| {
                            // get id of supporting cubes
                            cube_to_id_map.get(&IVec3::new(
                                cube.x,
                                cube.y,
                                cube.z - 1,
                            ))
                        })
                        .unique()
                        .count()
                        == 1
                })

                .count()
                == 0
        })
        .count();

    Ok(dissovable.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!("5", process(input)?);
        Ok(())
    }
}
