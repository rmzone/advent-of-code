use crate::{parse_input, Machine, Matrix};
use common::custom_error::Result;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, machines) = parse_input(input)?;

    let result = machines
        .iter()
        .map(|machine| calculate_pushes(machine))
        .sum::<usize>();

    Ok(result.to_string())
}

fn calculate_pushes(machine: &Machine) -> usize {
    let max_value = machine.joltages.iter().max().unwrap() + 1; // should not exceed this number of presses
    let mut matrix = Matrix::from_machine(&machine);
    matrix.reduce();
    // matrix.display();

    let mut min = usize::MAX;
    let mut values = vec![0; matrix.independents.len()];

    dfs(&matrix, 0, &mut values, &mut min, max_value);

    min
}

fn dfs(matrix: &Matrix, idx: usize, values: &mut [usize], min: &mut usize, max: usize) {
    // When we've assigned all independent variables, check if it's a valid solution.
    if idx == matrix.independents.len() {
        if let Some(total) = matrix.valid(values) {
            *min = (*min).min(total);
        }
        return;
    }

    // Try different values for the current independent variable.
    let total: usize = values[..idx].iter().sum();
    for val in 0..max {
        // Optimization: If we ever go above our min, we can't possibly do better.
        if total + val >= *min {
            break;
        }
        values[idx] = val;
        dfs(matrix, idx + 1, values, min, max);
    }
}

/*
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

10
One way to do this is by pressing
(3) once            0,0,0,1
(1,3) three times,  0,3,0,3
(2,3) three times,  0,0,3,3
(0,2) once,         1,0,1,0
(0,1) twice.        2,2,0,0

Converted to a matrix. columns are the buttons, rows are
 the lights that are lit when pushing a button

0 0 0 0 1 1 3
0 1 0 0 0 1 5
0 0 1 1 1 0 4
1 1 0 1 0 0 7

row echelon form: swap r0 and r3

1 1 0 1 0 0 7
0 1 0 0 0 1 5
0 0 1 1 1 0 4
0 0 0 0 1 1 3

reduced:
1 0 0 1 0 -1 2
0 1 0 0 0 1 5
0 0 1 1 0 -1 1
0 0 0 0 1 1 3

ass system of linear equations:
x0+x3-x5=2
x1+x5=5
x2+x3-x5=1
x4+x5=3

dependents: [0, 1, 2]
independents: [3]

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!("33", process(input)?);
        Ok(())
    }
}
