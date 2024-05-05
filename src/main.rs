#![allow(dead_code)]
mod tools;
use tools::table::Table;

fn main() {
    let mut table: Table<i32> = Table::from_file("data/1.txt");
    table.display(&table.costs());

    table.north_west_corner();

    table.display(&table.transport());
}
