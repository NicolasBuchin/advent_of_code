use std::{
    fmt,
    ops::{Index, IndexMut},
};

#[derive(Clone)]
pub struct Matrix<T: Clone> {
    pub data: Vec<T>,
    width: usize,
    height: usize,
}

#[allow(dead_code)]
impl<T: Default + Clone> Matrix<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Matrix {
            data: vec![T::default(); width * height],
            width,
            height,
        }
    }

    pub fn make(data: Vec<T>, width: usize, height: usize) -> Self {
        Matrix {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T: Clone> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        &self.data[(row * self.width)..(row * self.width + self.width)]
    }
}

impl<T: Clone> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[(row * self.width)..(row * self.width + self.width)]
    }
}

impl fmt::Display for Matrix<bool> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let symbol = if self[row][col] { '#' } else { '.' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?; // Add a newline after each row
        }
        Ok(())
    }
}
