use anyhow::{Ok, Result};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign},
    vec,
};

#[derive(Debug)]
pub struct Matrix<T: Debug + Default + Clone> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Debug + Default + Clone> Matrix<T> {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![T::default(); rows * cols],
            rows,
            cols,
        }
    }

    fn get(&self, i: usize, j: usize) -> &T {
        &self.data[i * self.cols + j]
    }

    fn set(&mut self, i: usize, j: usize, val: T) {
        self.data[i * self.cols + j] = val;
    }

    #[allow(dead_code)]
    fn with_data(mut self, data: impl Into<Vec<T>>) -> Self {
        self.data = data.into();
        self
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Mul<Output = T> + AddAssign + Add<Output = T> + MulAssign + Debug + Clone + Default,
{
    if a.cols != b.rows {
        return Err(anyhow::anyhow!("Matrix dimensions do not match"));
    }
    let mut result = Matrix::new(a.rows, b.cols);
    for i in 0..a.rows {
        for j in 0..b.cols {
            let mut sum = T::default();
            for k in 0..a.cols {
                sum += a.get(i, k).clone() * b.get(k, j).clone();
            }
            result.set(i, j, sum);
        }
    }
    Ok(result)
}

impl<T: Debug + Default + Clone> std::fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{:?}", self.get(i, j))?;
                if j < self.cols - 1 {
                    write!(f, " ")?;
                }
            }
            if i < self.rows - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")
    }
}

impl Mul for Matrix<usize> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        let a = Matrix::new(2, 3).with_data(vec![1, 2, 3, 4, 5, 6]);
        let b = Matrix::new(3, 2).with_data(vec![7, 8, 9, 10, 11, 12]);
        let c = multiply(&a, &b).unwrap();
        let c2 = a * b;
        assert_eq!(c.data, c2.data);
        assert_eq!(format!("{}", c), "{58 64, 139 154}");
    }

    #[test]
    fn test_display() {
        let a = Matrix::new(2, 3).with_data(vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(format!("{}", a), "{1 2 3, 4 5 6}");
    }
}
