#![allow(dead_code)]
mod tools;
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
            costs.set(i, j, die.sample(&mut rng));
            matrix.set(i, j, die.sample(&mut rng));
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
    // the objective is to benchmark the time it takes to solve the problem
    // using the north-west corner method

    let mut times = Vec::new();
    let nb_problems = 100;

    for _ in 0..nb_problems {
        let mut table: Table<u32> = generate_problem(1000, 1000);
        let start = std::time::Instant::now();
        table.north_west_corner();
        let elapsed = start.elapsed();
        times.push(elapsed);
    }

    println!(
        "Average time: {:?}",
        times.iter().sum::<std::time::Duration>() / nb_problems
    );
    println!("Worst time: {:?}", times.iter().max().unwrap());

    // display the solution
    let mut table: Table<u32> = generate_problem(10, 10);

    table.display(&table.costs());

    table.north_west_corner();

    table.display(&table.transport());
    println!("Total cost: {}", table.total_cost());
}
