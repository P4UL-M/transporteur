#![allow(dead_code)]
mod tools;
use std::env;

use rand::distributions::{Distribution, Uniform};
use tools::matrix::Matrix;
use tools::table::Table;

// generete problem of size n x m
// cost values are random between 1 and 100
// make two matrices, one for costs and one for transport supply and demand
// supply is the sum of the rows of the second matrix
// demand is the sum of the columns of the second matrix

fn generate_problem(n: usize, m: usize) -> Table<u32> {
    let mut costs = Matrix::new_empty(n, m);
    let mut matrix = Matrix::new_empty(n, m);

    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..100);

    for i in 0..n {
        for j in 0..m {
            costs[(i, j)] = die.sample(&mut rng);
            matrix[(i, j)] = die.sample(&mut rng);
        }
    }

    let supply = matrix.data().iter().map(|row| row.iter().sum()).collect();
    let demand = matrix
        .transpose()
        .data()
        .iter()
        .map(|row| row.iter().sum())
        .collect();

    Table::new(costs, Matrix::new_empty(n, m), supply, demand)
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    // the objective is to benchmark the time it takes to solve the problem
    // using the north-west corner method

    let mut times = Vec::new();
    let nb_problems = 100;
    let size = 400;

    for _ in 0..nb_problems {
        println!("Problem {}/{}", times.len() + 1, nb_problems);
        let mut table: Table<u32> = generate_problem(size, size);
        table.north_west_corner();
        let mut graph = table.get_graph();
        while !graph.is_connected() {
            graph
                .k_edge_augmentation(1, table.get_unused_edges())
                .unwrap();
        }
        let start = std::time::Instant::now();
        table.marginal_cost::<i64>(&graph);
        let elapsed = start.elapsed();
        times.push(elapsed);
    }

    println!(
        "Average time: {:?}",
        times.iter().sum::<std::time::Duration>() / nb_problems
    );
    println!("Worst time: {:?}", times.iter().max().unwrap());
}
