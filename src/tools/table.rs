use std::{
    fmt::{Debug, Display},
    iter::Sum,
    num::ParseIntError,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
    str::FromStr,
    vec,
};
use tabled::{
    builder::Builder,
    settings::{Alignment, Style},
};

use crate::tools::graph::Graph;
use crate::tools::matrix::Matrix;

use super::graph::Edge;

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
        + Display
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
        + Sum
        + PartialOrd
        + Div<Output = T>
        + SubAssign,
{
    pub fn new(costs: Matrix<T>, transport: Matrix<T>, supply: Vec<T>, demand: Vec<T>) -> Self {
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
        println!("n: {}, m: {}", n, m);
        let mut costs = Matrix::new_empty(n, m);
        let mut supply: Vec<T> = vec![Default::default(); n];
        let mut demand: Vec<T> = vec![Default::default(); m];
        for i in 0..n {
            let mut line = lines.next().unwrap().split_whitespace();
            for j in 0..m {
                costs[(i, j)] = line.next().unwrap().parse().unwrap();
            }
            supply[i] = line.next().unwrap().parse().unwrap();
        }
        let mut line = lines.next().unwrap().split_whitespace();
        for i in 0..m {
            demand[i] = line.next().unwrap().parse().unwrap();
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

    pub fn supply(&self) -> &Vec<T> {
        &self.supply
    }

    pub fn demand(&self) -> &Vec<T> {
        &self.demand
    }

    pub fn total_cost(&self) -> T {
        self.costs
            .data()
            .iter()
            .enumerate()
            .fold(Default::default(), |acc, (i, row)| {
                row.iter()
                    .enumerate()
                    .fold(acc, |acc, (j, &cost)| acc + cost * self.transport[(i, j)])
            })
    }

    pub fn north_west_corner(&mut self) {
        let mut i = 0;
        let mut j = 0;
        let mut supply = self.supply.clone();
        let mut demand = self.demand.clone();
        while i < self.n && j < self.m {
            let min = std::cmp::min(supply[i], demand[j]);
            self.transport[(i, j)] = min;
            supply[i] -= min;
            demand[j] -= min;
            if supply[i] == Default::default() {
                i += 1;
            }
            if demand[j] == Default::default() {
                j += 1;
            }
        }
    }

    pub fn get_graph(&self) -> Graph<T> {
        let mut graph = Graph::new();
        for i in 0..self.n {
            graph.add_node(format!("S{}", i + 1));
        }
        for j in 0..self.m {
            graph.add_node(format!("D{}", j + 1));
        }
        for i in 0..self.n {
            for j in 0..self.m {
                if self.transport[(i, j)] != Default::default() {
                    graph.add_edge(
                        format!("S{}", i + 1),
                        format!("D{}", j + 1),
                        self.transport[(i, j)],
                    );
                }
            }
        }
        graph
    }

    pub fn get_unused_edges(&self) -> Vec<Edge<T>> {
        let mut unused = Vec::new();
        for i in 0..self.n {
            for j in 0..self.m {
                if self.transport[(i, j)] == Default::default() {
                    unused.push(Edge::new(
                        format!("S{}", i + 1),
                        format!("D{}", j + 1),
                        self.costs[(i, j)],
                    ));
                }
            }
        }
        unused
    }

    pub fn display(&self, data: &Matrix<T>) {
        let mut table = Builder::default();

        let mut header = vec!["".to_string()];
        for j in 0..self.m {
            header.push(format!("D{}", j + 1));
        }
        header.push("Supply".to_string());
        table.push_record(header);

        // Add the costs matrix
        for i in 0..self.n {
            let mut row = Vec::new();
            row.push(format!("S{}", i + 1));
            for j in 0..self.m {
                let cost = data[(i, j)];
                row.push(cost.to_string());
            }
            // Add the supply value
            row.push(self.supply[i].to_string());
            table.push_record(row);
        }

        // Add the demand vector
        let mut row = vec!["Demand".to_string()];
        for j in 0..self.m {
            row.push(self.demand[j].to_string());
        }
        table.push_record(row);

        println!(
            "{}",
            table
                .build()
                .with(Style::rounded())
                .with(Alignment::center())
                .to_string()
        );
    }

    pub fn potentials<V>(&self, graph: &Graph<T>) -> (Vec<V>, Vec<V>)
    where
        V: Default
            + Clone
            + Copy
            + Add<Output = V>
            + Sub<Output = V>
            + Mul<Output = V>
            + Div<Output = V>
            + Ord
            + SubAssign
            + From<i8>,
        T: Into<V>,
    {
        let mut u = vec![Default::default(); self.n];
        let mut v = vec![Default::default(); self.m];

        // check if the graph is a tree
        if !graph.is_tree() {
            panic!("The graph is not a tree");
        }

        let size = self.n + self.m;

        let mut a: Matrix<i8> = Matrix::new_empty(size, size);
        let mut b: Vec<T> = vec![Default::default(); size];
        // fill the matrix A and the vector B with the edges and the costs
        let mut l = 0;
        for edge in graph.edges.iter() {
            if let (Some(i), Some(j)) = (
                edge.from
                    .strip_prefix("S")
                    .and_then(|s| s.parse::<usize>().ok()),
                edge.to
                    .strip_prefix("D")
                    .and_then(|s| s.parse::<usize>().ok()),
            ) {
                a[(l, i - 1)] = 1;
                a[(l, self.n + j - 1)] = -1;
                b[l] = self.costs[(i - 1, j - 1)];
                l += 1;
            }
        }
        // fill the last row of the matrix A
        a[(l, 0)] = 1;
        b[l] = 0.into();

        println!("Matrix A: {:?}", a);
        println!("Vector B: {:?}", b);

        // solve the system of linear equations
        let potentials = a.solve::<T, V>(&b);

        // fill the u and v vectors
        for i in 0..self.n {
            u[i] = potentials[i];
        }
        for j in 0..self.m {
            v[j] = potentials[self.n + j];
        }
        (u, v)
    }

    pub fn marginal_cost<V>(&self, graph: &Graph<T>) -> Matrix<V>
    where
        V: Default
            + Clone
            + Copy
            + Add<Output = V>
            + Sub<Output = V>
            + Mul<Output = V>
            + Div<Output = V>
            + Ord
            + SubAssign
            + From<i8>,
        T: Into<V>,
    {
        let (u, v) = self.potentials::<V>(graph);
        let mut potential = Matrix::new_empty(self.n, self.m);
        for i in 0..self.n {
            for j in 0..self.m {
                potential[(i, j)] = self.costs[(i, j)].into() - (u[i] - v[j]);
            }
        }
        potential
    }
}
