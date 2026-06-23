use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use std::cmp::PartialEq;

pub mod part1;
pub mod part2;

#[derive(Clone, PartialOrd, PartialEq)]
pub enum State {
    None,
    HasRule,
    SetDown,
    SetUp,
}

pub struct Firewall {
    pub width: usize,
    height: usize,
    data: Vec<Vec<State>>,
    rules: Vec<Rule>,
}

impl Firewall {
    pub fn new(rules: Vec<Rule>) -> Firewall {
        let width: usize = (rules.iter().map(|r| r.depth).max().unwrap_or(0) + 1) as usize;
        let height = (rules.iter().map(|r| r.range).max().unwrap_or(0)) as usize;
        let mut firewall = vec![vec![State::None; width]; height];
        for rule in &rules {
            let y = rule.depth as usize;
            for x in 0..rule.range as usize {
                if x == 0 {
                    firewall[x][y] = State::SetDown;
                } else {
                    firewall[x][y] = State::HasRule;
                }
            }
        }

        Firewall {
            width,
            height,
            data: firewall,
            rules,
        }
    }

    pub fn reset(&mut self) {
        self.data = vec![vec![State::None; self.width]; self.height];
        for rule in &self.rules {
            let y = rule.depth as usize;
            for x in 0..rule.range as usize {
                if x == 0 {
                    self.data[x][y] = State::SetDown;
                } else {
                    self.data[x][y] = State::HasRule;
                }
            }
        }
    }

    pub fn display(&self, position: i32) {
        for x in 0..self.width {
            print!(" {0}  ", x);
        }
        println!();

        for y in 0..self.height {
            for x in 0..self.width {
                let item = &self.data[y][x];
                if item == &State::SetDown || item == &State::SetUp {
                    if y == 0 && x == position as usize {
                        print!("(S) ");
                    } else {
                        print!("[S] ");
                    }
                } else if item == &State::HasRule {
                    if y == 0 && x == position as usize {
                        print!("( ) ");
                    } else {
                        print!("[ ] ");
                    }
                } else {
                    if y == 0 && x == position as usize {
                        print!("(.) ");
                    } else {
                        print!("    ");
                    }
                }
            }
            println!();
        }

        println!();
    }

    pub fn check_found(&self, p0: i32) -> i32 {
        if self.data[0][p0 as usize] == State::SetUp || self.data[0][p0 as usize] == State::SetDown
        {
            let rule = self.rules.iter().find(|r| r.depth == p0);
            if let Some(rule) = rule {
                return rule.depth * rule.range;
            }
        }
        0
    }

    pub fn check_found2(&self, p0: i32) -> i32 {
        if self.data[0][p0 as usize] == State::SetUp || self.data[0][p0 as usize] == State::SetDown
        {
            let rule = self.rules.iter().find(|r| r.depth == p0);
            if let Some(_) = rule {
                return 1;
            }
        }

        0
    }

    // not very efficient!
    pub fn move_packet(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let item = &self.data[y][x];

                if item == &State::SetDown {
                    if y + 1 >= self.height || &self.data[y + 1][x] != &State::HasRule {
                        // reverse direction
                        self.data[y][x] = State::HasRule;
                        self.data[y - 1][x] = State::SetUp;
                        break;
                    }

                    self.data[y][x] = State::HasRule;
                    self.data[y + 1][x] = State::SetDown;
                    break;
                } else if item == &State::SetUp {
                    if y as i32 - 1 < 0 {
                        // above there is always a rule
                        // reverse direction
                        self.data[y][x] = State::HasRule;
                        self.data[y + 1][x] = State::SetDown;
                        break;
                    }

                    self.data[y][x] = State::HasRule;
                    self.data[y - 1][x] = State::SetUp;
                    break;
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Rule {
    pub depth: i32,
    pub range: i32,
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(line_ending, parse_rule).parse(input)
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, (depth, range)) =
        separated_pair(complete::i32, tag(": "), complete::i32).parse(input)?;
    Ok((input, Rule { depth, range }))
}
