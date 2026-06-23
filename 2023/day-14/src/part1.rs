use common::custom_error::AocError;

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

            // check if can move north
            let mut y_north = y;
            while y_north > 0 {
                if map[y_north - 1][x] != Type::None {
                    break;
                }
                // move
                map[y_north][x] = Type::None;
                map[y_north - 1][x] = Type::RoundRock;
                y_north -= 1;
            }
        }
    }

    dbg!(&map);
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
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (mut map, width, height) = build_map(input);
    move_north(&mut map, width, height);
    let load = calculate_load(&mut map, width, height);

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
        assert_eq!("136", process(input)?);
        Ok(())
    }
}
