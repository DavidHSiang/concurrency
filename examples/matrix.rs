use anyhow::{Ok, Result};
use concurrency::{multiply, Matrix};

fn main() -> Result<()> {
    let a = Matrix::new(2, 2).with_data(vec![1, 2, 3, 4]);
    let b = Matrix::new(2, 2).with_data(vec![5, 6, 7, 8]);
    let c = multiply(&a, &b)?;
    println!("{}", c);
    Ok(())
}
