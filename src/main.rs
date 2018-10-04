#[macro_use]
extern crate text_io;

mod db;

use db::Table;

fn main() {
    let mut table: Table = Table::new();
    let exit_code = cli::run(&mut table);
    std::process::exit(exit_code);
}
