use common::custom_error::Result;
use common::parsing::Span;
use glam::IVec2;
use nom::branch::alt;
use nom::character::complete;
use nom::character::complete::{line_ending, multispace1};
use nom::combinator::value;
use nom::multi::{many1, separated_list1};
use nom::sequence::preceded;
use nom::IResult;
use nom_locate::position;
use ordermap::OrderSet;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone)]
enum Type {
    None,
    Robot,
    BoxLeft,
    BoxRight,
    Wall,
}

#[derive(Debug)]
struct Map {
    width: i32,
    height: i32,
    data: HashMap<IVec2, Type>,
}

impl Map {
    fn find_robot(&self) -> IVec2 {
        let robot = *self
            .data
            .iter()
            .find(|(_, v)| v == &&Type::Robot)
            .expect("A Robot")
            .0;

        robot
    }

    fn move_robot(&mut self, robot: &IVec2, direction: &IVec2) -> IVec2 {
        let mut cells: OrderSet<IVec2> = OrderSet::new();

        // see how far we can move. go to the first space or wall
        let mut position = *robot;
        cells.insert(position); // assume  that we can move the robot

        if direction.y == 0 {
            position = self.move_horizontal(&mut cells, robot, direction);
        } else {
            position = self.move_vertical(&mut cells, robot, direction);
        }

        // dbg!(&cells);

        // move any cells that can
        while let Some(cell) = cells.pop() {
            let temp = self.data.remove(&cell).unwrap_or(Type::None);
            self.data.insert(cell + *direction, temp);
        }

        position
    }

    fn move_horizontal(
        &mut self,
        cells: &mut OrderSet<IVec2>,
        robot: &IVec2,
        direction: &IVec2,
    ) -> IVec2 {
        let mut position = *robot;

        loop {
            let new_position = position + *direction;
            let cell = self.data.get(&new_position);
            if cell.is_none() {
                // we hit a free space, set new position of the robot.
                position = *robot + *direction;
                break;
            } else if cell.is_some_and(|c| c == &Type::Wall) {
                // we hit a wall so we cannot move anything
                cells.clear();
                position = *robot;
                break;
            }

            position = new_position;
            cells.insert(position);
        }

        position
    }

    fn move_vertical(
        &mut self,
        cells: &mut OrderSet<IVec2>,
        robot: &IVec2,
        direction: &IVec2,
    ) -> IVec2 {
        let position = *robot;
        let mut candidates: HashSet<IVec2> = HashSet::new();
        candidates.insert(position);

        loop {
            // println!("cells:      {:?}", cells);
            // println!("candidates: {:?}", candidates);
            let mut new_candidates: HashSet<IVec2> = HashSet::new();

            for candidate in candidates.iter() {
                let new_position = *candidate + *direction;
                let cell = self.data.get(&new_position);

                if cell.is_none() {
                    // position = *robot + *direction;
                    // break;
                    continue;
                } else if cell.is_some_and(|c| c == &Type::Wall) {
                    // we hit a wall so we cannot move anything
                    cells.clear();
                    return position;
                } else if cell.is_some_and(|c| c == &Type::BoxLeft) {
                    cells.insert(new_position);
                    cells.insert(IVec2::new(new_position.x + 1, new_position.y));

                    new_candidates.insert(new_position);
                    new_candidates.insert(IVec2::new(new_position.x + 1, new_position.y));
                } else if cell.is_some_and(|c| c == &Type::BoxRight) {
                    cells.insert(new_position);
                    cells.insert(IVec2::new(new_position.x - 1, new_position.y));

                    new_candidates.insert(new_position);
                    new_candidates.insert(IVec2::new(new_position.x - 1, new_position.y));
                }
            }

            if new_candidates.is_empty() {
                return position + *direction;
            }

            candidates = new_candidates;
        }
    }

    fn debug(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = match self.data.get(&IVec2::new(x, y)) {
                    None => ".",
                    Some(cell) => match cell {
                        Type::None => ".",
                        Type::BoxLeft => "[",
                        Type::BoxRight => "]",
                        Type::Wall => "#",
                        Type::Robot => "@",
                    },
                };
                print!("{}", cell);
            }
            println!();
        }
    }
}

pub fn process(input: &str) -> Result<String> {
    let new_input = input
        .chars()
        .map(|c| match c {
            '#' => "##".to_string(),
            'O' => "[]".to_string(),
            '.' => "..".to_string(),
            '@' => "@.".to_string(),
            other => other.to_string(),
        })
        .collect::<String>();

    let (_, (mut map, directions)) =
        parse_input(Span::new(new_input.as_str())).expect("Should parse!");
    let mut robot = map.find_robot();

    // println!("Start: {} {}", map.width, map.height);
    // map.debug();

    for direction in directions {
        robot = map.move_robot(&robot, &direction);
        // println!("\nrobot: {:?}, direction: {:?}", robot, direction);
        // map.debug();
    }

    map.debug();

    // calculate the gps of all the boxes
    let result = map
        .data
        .iter()
        .filter(|(_, v)| v == &&Type::BoxLeft)
        .map(|(pos, _)| pos.y * 100 + pos.x)
        .sum::<i32>();

    Ok(result.to_string())
}

fn parse_input(input: Span) -> IResult<Span, (Map, Vec<IVec2>)> {
    let (input, cells) = separated_list1(line_ending, many1(process_cell))(input)?;

    let (input, directions) = parse_commands(input)?;

    let data = cells
        .into_iter()
        .flatten()
        // .flatten()
        .filter(|(_, t)| t != &Type::None)
        .collect::<HashMap<IVec2, Type>>();

    let width = data.keys().map(|v| v.x).max().unwrap_or(0) + 1;
    let height = data.keys().map(|v| v.y).max().unwrap_or(0) + 1;

    Ok((
        input,
        (
            Map {
                width,
                height,
                data,
            },
            directions,
        ),
    ))
}

fn parse_commands(input: Span) -> IResult<Span, Vec<IVec2>> {
    let (input, directions) = preceded(
        multispace1,
        separated_list1(
            line_ending,
            many1(alt((
                value(IVec2::NEG_Y, complete::char('^')),
                value(IVec2::Y, complete::char('v')),
                value(IVec2::X, complete::char('>')),
                value(IVec2::NEG_X, complete::char('<')),
            ))),
        ),
    )(input)?;

    Ok((input, directions.into_iter().flatten().collect()))
}

fn process_cell(input: Span) -> IResult<Span, (IVec2, Type)> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, t) = alt((
        value(Type::None, complete::char('.')),
        value(Type::Wall, complete::char('#')),
        value(Type::BoxLeft, complete::char('[')),
        value(Type::BoxRight, complete::char(']')),
        value(Type::Robot, complete::char('@')),
    ))(input)?;

    Ok((input, (IVec2::new(x, y), t)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_small() -> Result<()> {
        let input = "#######
#...#.#
#.....#
#.@OO.#
#..O..#
#.....#
#######

>><vvv>^";
        assert_eq!("822", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process() -> Result<()> {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!("9021", process(input)?);
        Ok(())
    }
}
