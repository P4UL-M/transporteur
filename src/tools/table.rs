use std::{
    fmt::Debug,
    iter::Sum,
    num::ParseIntError,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
    str::FromStr,
};

use crate::tools::matrix::Matrix;

pub struct Table<T> {
    costs: Matrix<T>,
    transport: Matrix<T>,
    supply: Vec<T>,
    demand: Vec<T>,
    n: usize,
    m: usize,
}

impl<T> Table<T>
where
    T: Default
        + Clone
        + Copy
        + PartialEq
        + PartialOrd
        + AddAssign
        + Debug
        + From<u8>
        + Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + FromStr<Err = ParseIntError>
        + Ord
        + SubAssign
        + Copy
        + Sum,
{
    fn new(costs: Matrix<T>, transport: Matrix<T>, supply: Vec<T>, demand: Vec<T>) -> Self {
        // Check if the number of rows in the costs matrix is equal to the length of the supply vector
        assert_eq!(costs.rows(), supply.len());
        // Check if the number of columns in the costs matrix is equal to the length of the demand vector
        assert_eq!(costs.cols(), demand.len());
        // Check if the number of rows in the transport matrix is equal to the length of the supply vector
        assert_eq!(transport.rows(), supply.len());
        // Check if the number of columns in the transport matrix is equal to the length of the demand vector
        assert_eq!(transport.cols(), demand.len());

        let n = supply.len();
        let m = demand.len();

        Self {
            costs,
            transport,
            supply,
            demand,
            n,
            m,
        }
    }

    fn new_empty(n: usize, m: usize) -> Self {
        Self {
            costs: Matrix::new_empty(n, m),
            transport: Matrix::new_empty(n, m),
            supply: vec![Default::default(); n],
            demand: vec![Default::default(); m],
            n,
            m,
        }
    }

    pub fn from_file(filename: &str) -> Self {
        // file structure:
        // n m
        // c11 c12 ... c1m d1
        // c21 c22 ... c2m d2
        // ...
        // cn1 cn2 ... cnm dn
        // s1 s2 ... sm
        let file = std::fs::read_to_string(filename).unwrap();
        let mut lines = file.lines();
        let mut nm = lines.next().unwrap().split_whitespace();
        let n = nm.next().unwrap().parse().unwrap();
        let m = nm.next().unwrap().parse().unwrap();
        let mut costs = Matrix::new_empty(n, m);
        let mut supply: Vec<T> = vec![Default::default(); n];
        let mut demand: Vec<T> = vec![Default::default(); m];
        for i in 0..n {
            let mut line = lines.next().unwrap().split_whitespace();
            for j in 0..m {
                costs.set(i, j, line.next().unwrap().parse().unwrap());
            }
            demand[i] = line.next().unwrap().parse().unwrap();
        }
        let mut line = lines.next().unwrap().split_whitespace();
        for i in 0..m {
            supply[i] = line.next().unwrap().parse().unwrap();
        }
        // Check if there are no more lines in the file
        assert!(lines.next().is_none());
        // Check if the sum of the supply vector is equal to the sum of the demand vector
        assert!(
            supply.iter().copied().sum::<T>() == demand.iter().copied().sum::<T>(),
            "Supply and demand are not balanced"
        );
        Self::new(costs, Matrix::new_empty(n, m), supply, demand)
    }

    pub fn costs(&self) -> &Matrix<T> {
        &self.costs
    }

    pub fn transport(&self) -> &Matrix<T> {
        &self.transport
    }

    pub fn transport_mut(&mut self) -> &mut Matrix<T> {
        &mut self.transport
    }

    pub fn supply(&self) -> &Vec<T> {
        &self.supply
    }

    pub fn demand(&self) -> &Vec<T> {
        &self.demand
    }

    pub fn north_west_corner(&mut self) {
        let mut i = 0;
        let mut j = 0;
        while i < self.n && j < self.m {
            let min = std::cmp::min(self.supply[i], self.demand[j]);
            self.transport_mut().set(i, j, min);
            self.supply[i] -= min;
            self.demand[j] -= min;
            if self.supply[i] == Default::default() {
                i += 1;
            }
            if self.demand[j] == Default::default() {
                j += 1;
            }
        }
    }
}
