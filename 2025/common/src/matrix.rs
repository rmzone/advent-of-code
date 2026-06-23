
#[derive(Debug)]
pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
    pub matrix: Vec<Vec<f64>>,
    pub dependents: Vec<usize>,
    pub independents: Vec<usize>,
}

const EPSILON: f64 = 1e-9;

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            columns: cols,
            matrix: vec![],
            dependents: vec![],
            independents: vec![]
        }
    }

    pub fn reduce(&mut self) {
        // pivot until in row echelon form
        // 1) Interchanging two rows.
        // 2) Multiplying a row by a non-zero scalar.
        // 3) Adding a scalar multiple of one row to another.

        // (row / col we are reducing)
        let mut pivot = 0;
        let mut col = 0;

        while pivot < self.rows && col < self.columns {
            // Find the best pivot row for this column.
            let (best_row, best_value) = self
                .matrix
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
            self.matrix.swap(pivot, best_row);
            self.dependents.push(col);

            // Normalize pivot row.
            let pivot_value = self.matrix[pivot][col];
            for val in &mut self.matrix[pivot][col..=self.columns] {
                *val /= pivot_value;
            }

            // Eliminate this column in all other rows.
            for r in 0..self.rows {
                if r != pivot {
                    let factor = self.matrix[r][col];
                    if factor.abs() > EPSILON {
                        let pivot_row = self.matrix[pivot][col..=self.columns].to_vec();
                        self.matrix[r][col..=self.columns]
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
        self.independents.extend(col..self.columns);
    }

    pub fn display(&self) {
        for row in 0..self.rows {
            for col in 0..self.columns + 1 {
                print!("{} ", self.matrix[row][col]);
            }
            println!();
        }

        println!("dependents: {:?}", self.dependents);
        println!("independents: {:?}", self.independents);


        println!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test(){
        assert_eq!(Matrix::new(3, 4).display(), ());
    }
}
