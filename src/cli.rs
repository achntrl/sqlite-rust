use std::error::Error;
use std::fmt;
use std::io::{self, Write};

use db::{Row, Table, TABLE_MAX_ROWS};

use text_io;


static HELP_TEXT: &str = "
This is an SQLite clone written in Rust.

Available meta commands:
    .exit : Quit the interactive shell
    .help : Display this help message

Available statements:
    insert : Add a record in the database
    select : Display records from the database
";

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
    StringTooLong,
    Syntax,
    UnrecognizedStatement,
}

impl Error for PrepareError {
    fn description(&self) -> &str {
        match *self {
            PrepareError::StringTooLong => "Error: String is too long",
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
            PrepareError::StringTooLong => write!(f, "Error: String is too long"),
            PrepareError::Syntax => write!(f, "Error: Could not parse statement"),
            PrepareError::UnrecognizedStatement => {
                write!(f, "Error: Unrecognized keyword at start of statement")
            }
        }
    }
}

#[derive(Debug)]
pub enum ExecuteError {
    TableFull,
    UnrecognizedMetaCommand,
}

impl Error for ExecuteError {
    fn description(&self) -> &str {
        match *self {
            ExecuteError::TableFull => "Error: The table is full",
            ExecuteError::UnrecognizedMetaCommand => "Error: Unrecognized meta command",
        }
    }
}

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExecuteError::TableFull => write!(f, "Error: The table is full"),
            ExecuteError::UnrecognizedMetaCommand => write!(f, "Error: Unrecognized meta command"),
        }
    }
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

fn execute_meta_command(input_buffer: String) -> Result<Option<i32>, ExecuteError> {
    match input_buffer.trim().as_ref() {
        ".exit" => {
            Ok(Some(0))
        }
        ".help" => {
            println!("{}", HELP_TEXT);
            Ok(None)
        }
        _ => Err(ExecuteError::UnrecognizedMetaCommand),
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

            if username.len() > 32 || email.len() > 256 {
                return Err(PrepareError::StringTooLong);
            }

            let row: Row = Row {
                id,
                username,
                email,
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
            if table.num_row >= TABLE_MAX_ROWS {
                return Err(ExecuteError::TableFull);
            }
            let row_to_insert = statement.row.unwrap();
            table.insert_row(row_to_insert);
            Ok(())
        }
        StatementType::Select => {
            table.print_table();
            Ok(())
        }
    }
}

pub fn run(table: &mut Table) -> i32 {
    loop {
        print_prompt();
        let mut input_buffer = read_input();
        input_buffer = input_buffer.trim().to_string();

        if input_buffer.chars().next() == Some('.') {
            match execute_meta_command(input_buffer) {
                Ok(Some(exit_code)) => return exit_code,
                Ok(None) => continue,
                Err(e) => {
                    println!("{}.", e.description());
                    continue;
                }
            }
        }

        let statement = prepare_statement(input_buffer);

        match statement {
            Ok(statement) => {
                match execute_statement(statement, table) {
                    Ok(()) => println!("Executed."),
                    Err(e) => println!("{}.", e.description()),
                }
            }
            Err(e) => println!("{}.", e.description()),
        }
    }
}
