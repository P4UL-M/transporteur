#![allow(dead_code)]
mod tools;
use tools::table::Table;

fn main() {
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

    println!("{:?}", table.potentials::<i64>(graph));
}
