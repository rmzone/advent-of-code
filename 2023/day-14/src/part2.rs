use common::custom_error::AocError;
use tracing::info;

#[derive(Debug, PartialEq)]
enum Type {
    None,
    RoundRock,
    CubeRock,
}

fn build_map(input: &str) -> (Vec<Vec<Type>>, usize, usize) {
    let mut map: Vec<Vec<Type>> = Vec::new();
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        map.push(Vec::new());
        x = 0;
        for c in line.chars() {
            let s = match c {
                '.' => Type::None,
                'O' => Type::RoundRock,
                '#' => Type::CubeRock,
                _ => panic!("should never get here!"),
            };
            map[y].push(s);
            x += 1;
        }
        y += 1;
    }

    // dbg!(&map, &x, &y);

    (map, x, y)
}

fn move_north(map: &mut Vec<Vec<Type>>, width: usize, height: usize) {
    for y in 1..height {
        for x in 0..width {
            if map[y][x] != Type::RoundRock {
                continue;
            }

            let mut y_north = y;
            while y_north > 0 {
                if map[y_north - 1][x] != Type::None {
                    break;
                }

                map[y_north][x] = Type::None;
                map[y_north - 1][x] = Type::RoundRock;
                y_north -= 1;
            }
        }
    }

    // dbg!(&map);
}

fn move_south(map: &mut Vec<Vec<Type>>, width: usize, height: usize) {
    for y in (0..height - 1).rev() {
        for x in 0..width {
            if map[y][x] != Type::RoundRock {
                continue;
            }

            let mut y_north = y;
            while y_north < height - 1 {
                if map[y_north + 1][x] != Type::None {
                    break;
                }

                map[y_north][x] = Type::None;
                map[y_north + 1][x] = Type::RoundRock;
                y_north += 1;
            }
        }
    }

    // dbg!(&map);
}

fn move_east(map: &mut Vec<Vec<Type>>, width: usize, height: usize) {
    for y in 0..height {
        for x in (0..width - 1).rev() {
            if map[y][x] != Type::RoundRock {
                continue;
            }

            let mut x_west = x;
            while x_west < width - 1 {
                if map[y][x_west + 1] != Type::None {
                    break;
                }

                map[y][x_west] = Type::None;
                map[y][x_west + 1] = Type::RoundRock;
                x_west += 1;
            }
        }
    }

    // dbg!(&map);
}

fn move_west(map: &mut Vec<Vec<Type>>, width: usize, height: usize) {
    for y in 0..height {
        for x in 1..width {
            if map[y][x] != Type::RoundRock {
                continue;
            }

            let mut x_west = x;
            while x_west > 0 {
                if map[y][x_west - 1] != Type::None {
                    break;
                }

                map[y][x_west] = Type::None;
                map[y][x_west - 1] = Type::RoundRock;
                x_west -= 1;
            }
        }
    }

    // dbg!(&map);
}

fn calculate_load(map: &mut Vec<Vec<Type>>, width: usize, height: usize) -> i32 {
    let mut load = 0;

    for y in 0..height {
        for x in 0..width {
            if map[y][x] == Type::RoundRock {
                load += height - y;
            }
        }
    }

    load as i32
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str, iterations: i64) -> miette::Result<String, AocError> {
    let (mut map, width, height) = build_map(input);
    let mut load = 0;
    for i in 0..iterations {
        move_north(&mut map, width, height);
        move_west(&mut map, width, height);
        move_south(&mut map, width, height);
        move_east(&mut map, width, height);

        load = calculate_load(&mut map, width, height);

        dbg!(&i, &load);
    }

    Ok(load.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!("64", process(input, 1000)?);
        Ok(())
    }
}
