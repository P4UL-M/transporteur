#![allow(dead_code)]
mod tools;
use tools::table::Table;

fn main() {
    let table: Table<i32> = Table::from_file("data/1.txt");
    println!("{:?}", table.costs());
    println!("{:?}", table.transport());
}
