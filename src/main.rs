#[macro_use]
extern crate text_io;
extern crate byteorder;

mod db;
mod cli;
mod pager;

use db::Table;


fn main() {
    let mut table: Table = Table::new();
    let exit_code = cli::run(&mut table);
    table.close();
    std::process::exit(exit_code);
}
