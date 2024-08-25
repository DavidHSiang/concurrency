use anyhow::{Ok, Result};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign},
    sync::mpsc,
    thread, vec,
};

use crate::{dot_product, vector::Vector};

const NUM_THREADS: usize = 4;

#[derive(Debug)]
pub struct Matrix<T: Debug + Default + Clone> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

impl<T> MsgInput<T> {
    fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

pub struct MsgOutput<T> {
    idx: usize,
    val: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Msg<T> {
    fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

impl<T: Debug + Default + Clone> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
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
    pub fn with_data(mut self, data: impl Into<Vec<T>>) -> Self {
        self.data = data.into();
        self
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Mul<Output = T>
        + AddAssign
        + Add<Output = T>
        + MulAssign
        + Debug
        + Clone
        + Default
        + Send
        + 'static
        + Sync,
{
    if a.cols != b.rows {
        return Err(anyhow::anyhow!("Matrix dimensions do not match"));
    }
    // generate 4 threads which will calculate the dot product of each row and column
    // then sum the results

    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let val = dot_product(msg.input.row, msg.input.col)?;
                    msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        val,
                    })?;
                }
                Ok(())
            });
            tx
        })
        .collect::<Vec<_>>();

    let mut result = Matrix::new(a.rows, b.cols);
    let mut threads = vec![];
    for i in 0..a.rows {
        for j in 0..b.cols {
            let row = (0..a.cols)
                .map(|k| a.get(i, k).clone())
                .collect::<Vec<T>>()
                .into();
            let col = (0..b.rows)
                .map(|k| b.get(k, j).clone())
                .collect::<Vec<T>>()
                .into();

            let msg_input = MsgInput::new(i * b.cols + j, row, col);
            let (tx, rx) = oneshot::channel();
            senders[i % NUM_THREADS]
                .send(Msg::new(msg_input, tx))
                .expect("Failed to send message");
            threads.push(rx);
            // result.set(i, j, dot_product(row, col)?);
        }
    }
    for rx in threads {
        let msg_output = rx.recv()?;
        // result.data[msg_output.idx] = msg_output.val;
        result.set(
            msg_output.idx / b.cols,
            msg_output.idx % b.cols,
            msg_output.val,
        );
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

    #[test]
    fn test_multiply_error() {
        let a = Matrix::new(2, 3).with_data(vec![1, 2, 3, 4, 5, 6]);
        let b = Matrix::new(2, 2).with_data(vec![7, 8, 9, 10]);
        assert!(multiply(&a, &b).is_err());
    }

    #[test]
    #[should_panic]
    fn test_multiply_error_should_panic() {
        let a = Matrix::new(2, 3).with_data(vec![1, 2, 3, 4, 5, 6]);
        let b = Matrix::new(2, 2).with_data(vec![7, 8, 9, 10]);
        let _c = a * b;
    }
}
