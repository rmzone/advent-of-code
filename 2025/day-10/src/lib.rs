use nom::{
    branch::alt,
    character::complete::{self, line_ending, space1},
    multi::{fold_many1, separated_list1},
    sequence::delimited,
    IResult, Parser,
};

pub mod part1;
pub mod part2;

const EPSILON: f64 = 1e-9;

#[derive(Debug)]
pub struct Machine {
    pub lights: usize,
    pub buttons: Vec<Vec<usize>>,
    pub joltages: Vec<usize>,
}

#[derive(Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
    pub dependents: Vec<usize>,
    pub independents: Vec<usize>,
    is_reduced: bool,
}

impl Matrix {
    pub fn from_machine(machine: &Machine) -> Matrix {
        let rows = machine.joltages.len();
        let cols = machine.buttons.len();
        let mut data = vec![vec![0.0; cols + 1]; rows];

        // fill in buttons
        for (y, button) in machine.buttons.iter().enumerate() {
            for &item in button {
                if item < rows {
                    data[item][y] = 1.0;
                }
            }
        }

        // fill in joltages
        for (row, &val) in machine.joltages.iter().enumerate() {
            data[row][cols] = val as f64;
        }

        Matrix {
            rows,
            cols,
            data,
            dependents: Vec::new(),
            independents: Vec::new(),
            is_reduced: false,
        }
    }

    pub fn reduce(&mut self) {
        // pivot until in row echelon form
        // 1) Interchanging two rows.
        // 2) Multiplying a row by a non-zero scalar.
        // 3) Adding a scalar multiple of one row to another.

        let mut pivot = 0;
        let mut col = 0;

        while pivot < self.rows && col < self.cols {
            // Find the best pivot row for this column.
            let (best_row, best_value) = self
                .data
                .iter()
                .enumerate()
                .skip(pivot)
                .map(|(r, row)| (r, row[col].abs()))
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();

            // If the best value is zero, this is a free variable.
            if best_value < EPSILON {
                self.independents.push(col);
                col += 1;
                continue;
            }

            // Swap rows and mark this column as dependent.
            self.data.swap(pivot, best_row);
            self.dependents.push(col);

            // Normalize pivot row.
            let pivot_value = self.data[pivot][col];
            for val in &mut self.data[pivot][col..=self.cols] {
                *val /= pivot_value;
            }

            // Eliminate this column in all other rows.
            for r in 0..self.rows {
                if r != pivot {
                    let factor = self.data[r][col];
                    if factor.abs() > EPSILON {
                        let pivot_row = self.data[pivot][col..=self.cols].to_vec();
                        self.data[r][col..=self.cols]
                            .iter_mut()
                            .zip(&pivot_row)
                            .for_each(|(val, &pivot_val)| {
                                *val -= factor * pivot_val;
                            });
                    }
                }
            }

            pivot += 1;
            col += 1;
        }
        // Any remaining columns are free variables
        self.independents.extend(col..self.cols);

        self.is_reduced = true;
    }

    pub fn display(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols + 1 {
                print!("{} ", self.data[row][col]);
            }
            println!();
        }
        if self.is_reduced {
            println!("dependents: {:?}", self.dependents);
            println!("independents: {:?}", self.independents);
        }

        println!()
    }

    // Check if the given values for our independent variables are valid. If so, return the total button presses.
    pub fn valid(&self, values: &[usize]) -> Option<usize> {
        // We start with how many times we've pressed the free variables.
        let mut total = values.iter().sum::<usize>();

        // Calculate dependent variable values based on independent variables.
        for row in 0..self.dependents.len() {
            // Calculate this dependent by subtracting the sum of the free variable pushes from the solution.
            let val = self
                .independents
                .iter()
                .enumerate()
                .fold(self.data[row][self.cols], |acc, (i, &col)| {
                    acc - self.data[row][col] * (values[i] as f64)
                });

            // We need non-negative, whole numbers for a valid solution.
            if val < -EPSILON {
                return None;
            }
            let rounded = val.round();
            if (val - rounded).abs() > EPSILON {
                return None;
            }

            total += rounded as usize;
        }

        Some(total)
    }
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(line_ending, machine).parse(input)
}

fn machine(input: &str) -> IResult<&str, Machine> {
    let (input, lights) = lights(input)?;
    let (input, _) = space1(input)?;
    let (input, mut buttons) = separated_list1(space1, buttons).parse(input)?;
    let (input, _) = space1(input)?;
    let (input, joltages) = joltages(input)?;

    Ok((
        input,
        Machine {
            lights,
            buttons,
            joltages,
        },
    ))
}

fn lights(input: &str) -> IResult<&str, usize> {
    delimited(
        complete::char('['),
        fold_many1(
            alt((complete::char('.'), complete::char('#'))),
            || 0,
            |mut acc: usize, item| {
                acc = acc << 1;
                acc += match item {
                    '.' => 0,
                    '#' => 1,
                    _ => {
                        panic!("invalid!");
                    }
                };
                acc
            },
        ),
        complete::char(']'),
    )
    .parse(input)
}

fn buttons(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        complete::char('('),
        separated_list1(complete::char(','), complete::usize),
        complete::char(')'),
    )
    .parse(input)
}

fn joltages(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        complete::char('{'),
        separated_list1(complete::char(','), complete::usize),
        complete::char('}'),
    )
    .parse(input)
}
