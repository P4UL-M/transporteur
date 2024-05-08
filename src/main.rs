#![allow(dead_code)]
mod tools;
use std::env;

use tools::table::Table;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let mut table: Table<u32> = Table::from_file("data/7.txt");
    table.display(&table.costs());

    table.north_west_corner();

    table.display(&table.transport());

    let mut graph = table.get_graph();

    graph
        .k_edge_augmentation(1, table.get_unused_edges())
        .unwrap();

    println!("{:?}", graph);
    println!("{:?}", graph.is_tree());

    println!("Potentials : {:?}", table.potentials::<i64>(&graph));

    let matrix = table.marginal_cost::<i64>(&graph);
    let min_marginal_cost = matrix.min().unwrap();

    println!("Marginal cost : {:?}", matrix);
    println!("Min marginal cost : {:?}", min_marginal_cost);
    println!(
        "Index of min marginal cost : {:?}",
        matrix.index_of(min_marginal_cost).unwrap()
    );
}
