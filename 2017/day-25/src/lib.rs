use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, line_ending, space1};
use nom::combinator::map;
use nom::multi::many1;
use nom::{IResult, Parser};
use std::collections::HashMap;

pub mod part1;

#[derive(Debug)]
pub struct Machine {
    state: String,                // current state
    steps: usize,                 // current step count
    steps_check: usize,           // check after n-steps
    rules: HashMap<String, Rule>, // rules (key is state~value)
    tape_position: i64,
    tape: Vec<u8>, // tape
}

impl Machine {
    pub fn execute(&mut self) -> bool {
        // get rule from current state and value
        let current_value = self.tape[self.tape_position as usize];
        let key = format!("{}~{}", self.state, current_value);
        let rule = self.rules.get(&key).unwrap();

        // write
        self.tape[self.tape_position as usize] = rule.write;

        // move
        match rule.next_move {
            Direction::Left => {
                self.tape_position -= 1;
                if self.tape_position < 0 {
                    self.tape_position = 0;
                    self.tape.insert(0, 0);
                }
            }
            Direction::Right => {
                self.tape_position += 1;
                if self.tape_position >= self.tape.len() as i64 {
                    self.tape.push(0);
                }
            }
        }

        // next state
        self.state = rule.next_state.clone();

        self.steps += 1;
        if self.steps >= self.steps_check {
            return true;
        }

        false
    }
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Rule {
    write: u8,
    next_move: Direction,
    next_state: String,
}

pub fn parse_input(input: &str) -> IResult<&str, Machine> {
    let (input, (_, state, _, _)) =
        (tag("Begin in state "), alpha1, tag("."), line_ending).parse(input)?;
    let (input, (_, steps, _, _)) = (
        tag("Perform a diagnostic checksum after "),
        complete::usize,
        tag(" steps."),
        line_ending,
    )
        .parse(input)?;
    let (input, rules) = rules(input)?;

    let machine = Machine {
        state: state.to_string(),
        steps: 0,
        steps_check: steps,
        rules,
        tape: vec![0],
        tape_position: 0,
    };

    Ok((input, machine))
}

fn rules(input: &str) -> IResult<&str, HashMap<String, Rule>> {
    let (input, rules) = many1(rule).parse(input)?;
    let result = rules.into_iter().flatten().collect::<HashMap<_, _>>();
    Ok((input, result))
}

fn rule(input: &str) -> IResult<&str, Vec<(String, Rule)>> {
    let (input, _) = line_ending(input)?;
    let (input, (_, state, _, _)) =
        (tag("In state "), alpha1, tag(":"), line_ending).parse(input)?;
    let (input, (_, _, _, condition0, _, _)) = (
        space1,
        tag("If the current value is"),
        space1,
        complete::usize,
        tag(":"),
        line_ending,
    )
        .parse(input)?;
    let (input, (_, write0, _, _)) = (
        tag("    - Write the value "),
        complete::u8,
        tag("."),
        line_ending,
    )
        .parse(input)?;
    let (input, (_, direction0, _, _)) = (
        tag("    - Move one slot to the "),
        alt((
            map(tag("left"), |_| Direction::Left),
            map(tag("right"), |_| Direction::Right),
        )),
        tag("."),
        line_ending,
    )
        .parse(input)?;
    let (input, (_, next_state0, _, _)) = (
        tag("    - Continue with state "),
        alpha1,
        tag("."),
        line_ending,
    )
        .parse(input)?;
    let (input, (_, _, _, condition1, _, _)) = (
        space1,
        tag("If the current value is"),
        space1,
        complete::usize,
        tag(":"),
        line_ending,
    )
        .parse(input)?;
    let (input, (_, write1, _, _)) = (
        tag("    - Write the value "),
        complete::u8,
        tag("."),
        line_ending,
    )
        .parse(input)?;
    let (input, (_, direction1, _, _)) = (
        tag("    - Move one slot to the "),
        alt((
            map(tag("left"), |_| Direction::Left),
            map(tag("right"), |_| Direction::Right),
        )),
        tag("."),
        line_ending,
    )
        .parse(input)?;
    let (input, (_, next_state1, _, _)) = (
        tag("    - Continue with state "),
        alpha1,
        tag("."),
        line_ending,
    )
        .parse(input)?;

    let mut rules = Vec::new();
    rules.push((
        format!("{}~{}", state.to_string(), condition0),
        Rule {
            write: write0,
            next_move: direction0,
            next_state: next_state0.to_owned(),
        },
    ));
    rules.push((
        format!("{}~{}", state.to_string(), condition1),
        Rule {
            write: write1,
            next_move: direction1,
            next_state: next_state1.to_owned(),
        },
    ));

    Ok((input, rules))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_parse_input() {
        let mut expected = Machine {
            state: "A".to_string(),
            steps: 0,
            steps_check: 6,
            rules: HashMap::new(),
            tape: vec![0],
            tape_position: 0,
        };

        expected.rules.insert(
            "A~0".to_string(),
            Rule {
                write: 1,
                next_move: Direction::Right,
                next_state: "B".to_string(),
            },
        );

        expected.rules.insert(
            "A~1".to_string(),
            Rule {
                write: 0,
                next_move: Direction::Left,
                next_state: "B".to_string(),
            },
        );

        expected.rules.insert(
            "B~0".to_string(),
            Rule {
                write: 1,
                next_move: Direction::Left,
                next_state: "A".to_string(),
            },
        );

        expected.rules.insert(
            "B~1".to_string(),
            Rule {
                write: 0,
                next_move: Direction::Right,
                next_state: "A".to_string(),
            },
        );

        let input = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
";

        let actual = parse_input(input).unwrap();
        assert_eq!(expected.state, actual.1.state);
        assert_eq!(expected.steps_check, actual.1.steps_check);
        assert_eq!(expected.rules.len(), actual.1.rules.len());
    }

    #[test]
    fn test_parse_rule() {
        let input = "
In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.
";

        let actual = rule(input).unwrap();
        assert_eq!(2, actual.1.len());
    }
}
