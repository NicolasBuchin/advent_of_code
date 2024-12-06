use std::{
    fmt,
    ops::{Index, IndexMut},
};

#[derive(Clone)]
pub struct Matrix<T: Clone> {
    pub data: Box<[T]>,
    width: usize,
    height: usize,
}

#[allow(dead_code)]
impl<T: Default + Clone> Matrix<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Matrix {
            data: vec![T::default(); width * height].into_boxed_slice(),
            width,
            height,
        }
    }

    pub fn make(data: &[T], width: usize, height: usize) -> Self {
        Matrix {
            data: Box::from(data),
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

impl<T: fmt::Display + Clone> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            let start = row * self.width;
            let end = start + self.width;
            let row_data = &self.data[start..end];
            let row_string = row_data
                .iter()
                .map(|item| format!("{}", item))
                .collect::<Vec<_>>()
                .join(" ");
            writeln!(f, "{}", row_string)?;
        }
        Ok(())
    }
}

impl<T: fmt::Debug + Clone> fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix {{ width: {}, height: {} }}", self.width, self.height)?;
        for row in 0..self.height {
            let start = row * self.width;
            let end = start + self.width;
            let row_data = &self.data[start..end];
            writeln!(f, "{:?}", row_data)?;
        }
        Ok(())
    }
}
