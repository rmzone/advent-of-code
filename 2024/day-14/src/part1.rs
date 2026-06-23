use common::custom_error::Result;
use glam::IVec2;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Robot {
    position: IVec2,
    velocity: IVec2,
}

pub fn process(input: &str, width: i32, height: i32) -> Result<String> {
    let (_, mut robots) = process_input(input).expect("Should parse!");

    // position 0,0 is top left and grows down and to the right
    for _ in 0..100 {
        // update robots positions for 100 iterations

        for robot in robots.iter_mut() {
            robot.position.x = (robot.position.x + robot.velocity.x) % width;
            robot.position.y = (robot.position.y + robot.velocity.y) % height;

            if robot.position.x < 0 {
                robot.position.x += width;
            }

            if robot.position.y < 0 {
                robot.position.y += height;
            }

            // println!("{:?}", &robot);
        }
    }

    // calculate count of robots by quadrent
    let mut count_map = HashMap::new();
    for robot in robots.iter() {
        if robot.position.x == width / 2 || robot.position.y == height / 2 {
            continue;
        }

        let middle_x = width / 2;
        let middle_y = height / 2;

        if robot.position.x < middle_x {
            if robot.position.y < middle_y {
                count_map.entry(0).and_modify(|e| *e += 1).or_insert(1);
            } else {
                count_map.entry(1).and_modify(|e| *e += 1).or_insert(1);
            }
        } else {
            if robot.position.y < middle_y {
                count_map.entry(2).and_modify(|e| *e += 1).or_insert(1);
            } else {
                count_map.entry(3).and_modify(|e| *e += 1).or_insert(1);
            }
        }
    }

    println!("{:?}", count_map);

    let result: i32 = count_map.iter().map(|(_, &v)| v).product();

    Ok(result.to_string())
}

fn process_input(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(line_ending, robot)(input)
}

fn robot(input: &str) -> IResult<&str, Robot> {
    let (input, (position, velocity)) =
        separated_pair(preceded(tag("p="), pair), space1, preceded(tag("v="), pair))(input)?;

    Ok((input, Robot { position, velocity }))
}

fn pair(input: &str) -> IResult<&str, IVec2> {
    let (input, (a, b)) = separated_pair(complete::i32, tag(","), complete::i32)(input)?;
    Ok((input, IVec2::new(a, b)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("12", process(input, 11, 7)?);
        Ok(())
    }

    #[test]
    fn test_process_one_robot() -> Result<()> {
        let input = "p=2,4 v=2,-3";
        assert_eq!("1", process(input, 11, 7)?);
        Ok(())
    }

    #[test]
    fn test_robot() {
        let input = "p=0,4 v=3,-3";
        assert_eq!(
            Ok((
                "",
                Robot {
                    position: IVec2::new(0, 4),
                    velocity: IVec2::new(3, -3)
                }
            )),
            robot(input)
        );
    }

    #[test]
    fn test_pair() {
        let input = "0,4";
        assert_eq!(Ok(("", IVec2::new(0, 4))), pair(input));
    }
}
