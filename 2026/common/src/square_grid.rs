use std::hash::{Hash, Hasher};
use std::ops::{Index, IndexMut};

// from https://github.com/rene-d/advent-of-rust/blob/main/crates/aoc/src/square.rs

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SquareGrid<T> {
    cells: Vec<T>,
    size: usize,
    dummy: T,
}

impl<T: Clone + Copy + Default> SquareGrid<T> {
    /// Create a new empty square.
    #[must_use]
    pub fn new(size: usize) -> Self {
        Self {
            cells: vec![T::default(); size * size],
            size,
            dummy: T::default(),
        }
    }

    /// Return the size of the square (not the number of cells).
    #[must_use]
    pub const fn size(&self) -> usize {
        self.size
    }

    /// Return the cells as a flattened slice.
    pub fn values(&self) -> &[T] {
        &self.cells
    }

    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.cells.iter().enumerate().map(move |(i, c)| {
            let x = i % self.size;
            let y = i / self.size;
            ((x, y), c)
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> {
        self.cells.iter_mut().enumerate().map(|(i, c)| {
            let x = i % self.size;
            let y = i / self.size;
            ((x, y), c)
        })
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
        (0..self.size).map(|i| &self.cells[(i * self.size)..((i + 1) * self.size)])
    }

    pub fn iter_combos(&self) -> impl Iterator<Item = Self> + '_ {
        let mut square = self.clone();
        (0..8).map(move |i| {
            match i {
                0 => {}
                1..=3 | 5..8 => square.rotate_inplace(),
                4 => {
                    square.rotate_inplace();
                    square.flip_vertical_inplace();
                }
                _ => panic!(),
            }
            square.clone()
        })
    }

    /// Clockwise rotation
    pub fn rotate_inplace(&mut self) {
        let orig = self.clone();
        for x in 0..self.size {
            for y in 0..self.size {
                self[(x, y)] = orig[(y, self.size - 1 - x)];
            }
        }
    }

    /// Symmetry about the Y axis
    pub fn flip_vertical_inplace(&mut self) {
        let n = self.size;
        for y in 0..n {
            for x in 0..(n / 2) {
                let tmp = self[(x, y)];
                self[(x, y)] = self[(n - 1 - x, y)];
                self[(n - 1 - x, y)] = tmp;
            }
        }
    }

    /// Extract a subsquare from a bigger one.
    pub fn get_square(&self, offset_x: usize, offset_y: usize, size: usize) -> Self {
        let mut subsquare = Self::new(size);
        for y in 0..size {
            for x in 0..size {
                if offset_x + x < self.size && offset_y + y < self.size {
                    subsquare[(x, y)] = self[(offset_x + x, offset_y + y)];
                }
            }
        }
        subsquare
    }

    /// Fill a square from a subsquare
    pub fn put_square(&mut self, offset_x: usize, offset_y: usize, subsquare: &Self) {
        for y in 0..subsquare.size {
            for x in 0..subsquare.size {
                if offset_x + x < self.size && offset_y + y < self.size {
                    self[(offset_x + x, offset_y + y)] = subsquare[(x, y)];
                }
            }
        }
    }
}

/// Get the element (x,y) of a square.
impl<T> Index<(usize, usize)> for SquareGrid<T> {
    type Output = T;
    #[inline]
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        if x < self.size && y < self.size {
            &self.cells[self.size * y + x]
        } else {
            &self.dummy
        }
    }
}

/// Set the element (x,y) of a square.
impl<T> IndexMut<(usize, usize)> for SquareGrid<T> {
    #[inline]
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        if x < self.size && y < self.size {
            &mut self.cells[self.size * y + x]
        } else {
            &mut self.dummy
        }
    }
}

impl<T: Hash> Hash for SquareGrid<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&*self.cells, state);
    }
}

impl<T: std::fmt::Display> std::fmt::Display for SquareGrid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size {
            for x in 0..self.size {
                let idx = self.size * y + x;
                write!(f, "{}", self.cells[idx])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
