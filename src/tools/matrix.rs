use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

// Create a struct Matrix with a field data of type Vec<Vec<T>>.
#[derive(Debug, Clone)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T>
where
    T: Default + Clone + Copy,
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

    pub fn is_square(&self) -> bool {
        self.rows() == self.cols()
    }

    pub fn is_empty(&self) -> bool {
        self.rows() == 0 || self.cols() == 0
    }

    pub fn min(&self) -> Option<T>
    where
        T: Ord,
    {
        self.data.iter().flatten().copied().min()
    }

    pub fn index_of(&self, value: T) -> Option<(usize, usize)>
    where
        T: PartialEq,
    {
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                if self.data[i][j] == value {
                    return Some((i, j));
                }
            }
        }
        None
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &Vec<T>> {
        self.data.iter()
    }

    pub fn iter_cols(&self) -> impl Iterator<Item = Vec<T>> + '_ {
        (0..self.cols()).map(move |j| {
            (0..self.rows())
                .map(move |i| self.data[i][j])
                .collect::<Vec<T>>()
        })
    }
}

impl<T> Matrix<T>
where
    T: Default + Clone + Copy + Debug,
{
    pub fn solve<U, V>(&self, b: &Vec<U>) -> Vec<V>
    where
        V: Default
            + Clone
            + Copy
            + From<T>
            + Ord
            + Div<Output = V>
            + Mul<Output = V>
            + SubAssign
            + Debug
            + Neg<Output = V>,
        U: Into<V> + Copy,
    {
        let mut augmented: Matrix<V> = Matrix::new_empty(self.rows(), self.cols() + 1);
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                augmented.data[i][j] = self.data[i][j].into();
            }
            augmented.data[i][self.cols()] = b[i].into();
        }

        let mut i = 0;
        let mut j = 0;
        while i < augmented.rows() && j < augmented.cols() {
            let mut max: V = Default::default();
            let mut kmax = i;
            for k in i + 1..augmented.rows() {
                if augmented.data[k][j] > max {
                    kmax = k;
                    max = augmented.data[k][j];
                } else if -augmented.data[k][j] > max {
                    kmax = k;
                    max = -augmented.data[k][j];
                }
            }
            augmented.data.swap(i, kmax);
            let pivot = augmented.data[i][j];
            if pivot == V::default() {
                panic!("Matrix is singular");
            }
            for k in i + 1..augmented.rows() {
                let factor = augmented.data[k][j] / pivot;
                for l in j..augmented.cols() {
                    let val = augmented[(i, l)] * factor;
                    augmented.data[k][l] -= val;
                }
            }
            i += 1;
            j += 1;
        }

        let mut solution: Vec<V> = vec![Default::default(); self.cols()];
        for i in (0..self.cols()).rev() {
            solution[i] = augmented.data[i][self.cols()] / augmented.data[i][i];
            for j in 0..i {
                let val = augmented.data[j][i] * solution[i];
                augmented.data[j][self.cols()] -= val;
            }
        }
        solution
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
    T: Add<Output = T> + Clone + Default + Copy,
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
    T: Sub<Output = T> + Clone + Default + Copy,
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
    T: Default + Clone + Add<Output = T> + Mul<Output = T> + Copy + AddAssign,
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
    T: Default + Clone + Add<Output = T> + Mul<Output = T> + Copy,
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

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}
