use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::Sub;

// Create a struct Matrix with a field data of type Vec<Vec<T>>.
#[derive(Debug, Clone)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T>
where
    T: Default + Clone + From<u8> + Copy,
{
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Self {
            rows: data.len(),
            cols: data[0].len(),
            data,
        }
    }

    pub fn new_empty(n: usize, m: usize) -> Self {
        Self {
            rows: n,
            cols: m,
            data: vec![vec![Default::default(); m]; n],
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn data(&self) -> &Vec<Vec<T>> {
        &self.data
    }

    pub fn transpose(&self) -> Self {
        let mut transposed = vec![vec![Default::default(); self.rows()]; self.cols()];
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                transposed[j][i] = self.data[i][j];
            }
        }
        Self::new(transposed)
    }

    pub fn identity(&self) -> Self {
        let mut identity = vec![vec![Default::default(); self.cols()]; self.rows()];
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                if i == j {
                    identity[i][j] = T::from(1);
                }
            }
        }
        Self::new(identity)
    }

    pub fn is_square(&self) -> bool {
        self.rows() == self.cols()
    }

    pub fn is_empty(&self) -> bool {
        self.rows() == 0 || self.cols() == 0
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&T> {
        self.data.get(i).and_then(|row| row.get(j))
    }

    pub fn set(&mut self, i: usize, j: usize, value: T) {
        self.data[i][j] = value;
    }
}

impl<T> Display for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.data {
            for col in row {
                write!(f, "{} ", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> PartialEq for Matrix<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T> Add for Matrix<T>
where
    T: Add<Output = T> + Clone + Default + Copy + From<u8>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut sum = vec![vec![Default::default(); self.cols()]; self.rows()];
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                sum[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        Self::new(sum)
    }
}

impl<T> Sub for Matrix<T>
where
    T: Sub<Output = T> + Clone + Default + Copy + From<u8>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut diff = vec![vec![Default::default(); self.cols()]; self.rows()];
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                diff[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        Self::new(diff)
    }
}

impl<T> Mul<Matrix<T>> for Matrix<T>
where
    T: Default + Clone + Add<Output = T> + Mul<Output = T> + Copy + From<u8> + AddAssign,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut product = vec![vec![Default::default(); other.cols()]; self.rows()];
        for i in 0..self.rows() {
            for j in 0..other.cols() {
                for k in 0..self.cols() {
                    product[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        Self::new(product)
    }
}

impl<T> Mul<T> for Matrix<T>
where
    T: Default + Clone + Add<Output = T> + Mul<Output = T> + Copy + From<u8>,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        let mut product = vec![vec![Default::default(); self.cols()]; self.rows()];
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                product[i][j] = self.data[i][j] * scalar;
            }
        }
        Self::new(product)
    }
}
