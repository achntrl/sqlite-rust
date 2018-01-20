#![feature(const_size_of)]

use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use std::mem;

#[macro_use]
extern crate text_io;
extern crate arrayvec;

use arrayvec::ArrayString;

const ROW_SIZE: usize = mem::size_of::<Row>();
const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 32;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

enum StatementType {
    Insert,
    Select,
}

struct Statement {
    statement_type: StatementType,
    row: Option<Row>,
}

#[derive(Debug)]
enum PrepareError {
    Syntax,
    UnrecognizedStatement,
}

impl Error for PrepareError {
    fn description(&self) -> &str {
        match *self {
            PrepareError::Syntax => "Error: Could not parse statement",
            PrepareError::UnrecognizedStatement => {
                return "Error: Unrecognized keyword at start of statement";
            }
        }
    }
}

impl fmt::Display for PrepareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PrepareError::Syntax => write!(f, "Error: Could not parse statement"),
            PrepareError::UnrecognizedStatement => {
                write!(f, "Error: Unrecognized keyword at start of statement")
            }
        }
    }
}

#[derive(Debug)]
enum ExecuteError {
    TableFull,
    UnrecognizedMetaCommand
}

impl Error for ExecuteError {
    fn description(&self) -> &str {
        match *self {
            ExecuteError::TableFull => "Error: The table is full",
            ExecuteError::UnrecognizedMetaCommand => "Error: Unrecognized meta command"

        }
    }
}

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExecuteError::TableFull => write!(f, "Error: The table is full"),
            ExecuteError::UnrecognizedMetaCommand => write!(f, "Error: Unrecognized meta command")
        }
    }
}

static HELP_TEXT: &str = "
This is an SQLite clone written in Rust.

Available meta commands:
    .exit : Quit the interactive shell
    .help : Display this help message

Available statements:
    insert : Add a record in the database
    select : Display records from the database
";

#[derive(Debug, Clone)]
struct Row {
    id: u32,
    username: ArrayString<[u8; 32]>,
    email: ArrayString<[u8; 256]>,
}

#[derive(Default, Debug)]
struct Page {
    rows: [Option<Row>; ROWS_PER_PAGE],
}

#[derive(Default, Debug)]
struct Table {
    pages: [Option<Page>; TABLE_MAX_PAGES],
    num_rows: usize,
}


fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}

fn read_input() -> String {
    let mut input_buffer = String::new();

    io::stdin()
        .read_line(&mut input_buffer)
        .expect("Failed to read stdin");

    input_buffer
}

fn execute_meta_command(input_buffer: String) -> Result<(), ExecuteError> {
    match input_buffer.trim().as_ref() {
        ".exit" => {
            std::process::exit(0);
        }
        ".help" => {
            println!("{}", HELP_TEXT);
            Ok(())
        }
        _ => {
            Err(ExecuteError::UnrecognizedMetaCommand)
        }
    }
}

fn prepare_statement(input_buffer: String) -> Result<Statement, PrepareError> {
    let statement: &str = input_buffer.trim().as_ref();
    match statement {
        _ if statement.starts_with("insert") => {
            let mut id: u32;
            let mut username: String;
            let mut email: String;
            let scan_result = parse_insert(statement);
            match scan_result {
                Ok((_id, _username, _email)) => {
                    id = _id;
                    username = _username;
                    email = _email;
                }
                Err(_err) => return Err(PrepareError::Syntax),
            };
            let row: Row = Row {
                id,
                username: ArrayString::<[u8; 32]>::from(username.as_str()).unwrap(),
                email: ArrayString::<[u8; 256]>::from(email.as_str()).unwrap(),
            };
            Ok(Statement {
                   statement_type: StatementType::Insert,
                   row: Some(row),
               })
        }
        _ if statement.starts_with("select") => {
            Ok(Statement {
                   statement_type: StatementType::Select,
                   row: Default::default(),
               })
        }
        _ => Err(PrepareError::UnrecognizedStatement),
    }
}

fn parse_insert(statement: &str) -> Result<(u32, String, String), text_io::Error> {
    let id: u32;
    let username: String;
    let email: String;
    try_scan!(statement.bytes() => "insert {} {} {}", id, username, email);
    Ok((id, username, email))
}

fn execute_statement(statement: Statement, table: &mut Table) -> Result<(), ExecuteError> {
    match statement.statement_type {
        StatementType::Insert => {
            if table.num_rows > TABLE_MAX_ROWS {
                return Err(ExecuteError::TableFull);
            }
            let row_to_insert = statement.row.unwrap();
            insert_row(row_to_insert, table);
            table.num_rows += 1;
            Ok(())
        }
        StatementType::Select => {
            print_table(table);
            Ok(())
        }
    }
}

fn insert_row(row_to_insert: Row, table: &mut Table) {
    let row = row_to_insert.clone();
    let page_num: usize = table.num_rows / ROWS_PER_PAGE;
    match table.pages[page_num] {
        Some(ref mut page) => {
            let row_offset = table.num_rows % ROWS_PER_PAGE;
            page.rows[row_offset] = Some(row);
        }
        None => {
            let mut page: Page = Default::default();
            page.rows[0] = Some(row);
            table.pages[page_num] = Some(page);
        }
    };
}

fn print_table(table: &Table) {
    for some_page in table.pages.iter() {
        match some_page {
            &Some(ref page) => {
                for some_row in page.rows.iter() {
                    match some_row {
                        &Some(ref row) => {
                            print_row(row);
                        }
                        &None => break,
                    }
                }
            }
            &None => break,
        }
    }
}

fn print_row(row: &Row) {
    println!("({}, {}, {})", row.id, row.username, row.email);
}

fn main() {
    let mut table: Table = Default::default();

    loop {
        print_prompt();
        let mut input_buffer = read_input();
        input_buffer = input_buffer.trim().to_string();

        if input_buffer.chars().next() == Some('.') {
            match execute_meta_command(input_buffer) {
                Ok(()) => continue,
                Err(e) => {
                    println!("{}.", e.description());
                    continue;
                }
            }
        }

        let statement = prepare_statement(input_buffer);

        match statement {
            Ok(statement) => {
                match execute_statement(statement, &mut table) {
                    Ok(()) => println!("Executed."),
                    Err(e) => println!("{}.", e.description()),
                }
            }
            Err(e) => println!("{}.", e.description()),
        }
    }
}
