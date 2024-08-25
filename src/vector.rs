use anyhow::{Ok, Result};
use std::ops::Deref;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign},
};

pub struct Vector<T> {
    pub data: Vec<T>,
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> From<Vec<T>> for Vector<T> {
    fn from(data: Vec<T>) -> Self {
        Self { data }
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Mul<Output = T> + AddAssign + Add<Output = T> + MulAssign + Debug + Clone + Default,
{
    if a.len() != b.len() {
        return Err(anyhow::anyhow!("Vector dimensions do not match"));
    }
    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i].clone() * b[i].clone();
    }
    Ok(sum)
}
