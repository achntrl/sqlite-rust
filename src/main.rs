use std::io::{self, Write};

enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

enum StatementType {
    Insert,
    Select,
}

struct Statement {
    statement_type: StatementType,
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

fn execute_meta_command(input_buffer: String) -> MetaCommandResult {
    match input_buffer.trim().as_ref() {
        ".exit" => {
            std::process::exit(0);
        }
        ".help" => {
            println!("{}", HELP_TEXT);
            MetaCommandResult::Success
        }
        _ => {
            println!("Cannot execute this meta command");
            MetaCommandResult::UnrecognizedCommand
        }
    }
}

fn prepare_statement(input_buffer: String) -> Option<Statement> {
    let s: &str = input_buffer.trim().as_ref();
    match s {
        _ if s.starts_with("insert") => Some(Statement { statement_type: StatementType::Insert }),
        _ if s.starts_with("select") => Some(Statement { statement_type: StatementType::Select }),
        _ => None,
    }
}

fn execute_statement(statement: Statement) {
    match statement.statement_type {
        StatementType::Insert => println!("This is where we execute the insert statement"),
        StatementType::Select => println!("This is where we execute the select statement"),
    }
}

fn main() {
    loop {
        print_prompt();
        let mut input_buffer = read_input();
        input_buffer = input_buffer.trim().to_string();

        if input_buffer.chars().next() == Some('.') {
            match execute_meta_command(input_buffer) {
                MetaCommandResult::Success => continue,
                MetaCommandResult::UnrecognizedCommand => continue,
            }
        }

        let statement = prepare_statement(input_buffer);

        match statement {
            Some(statement) => execute_statement(statement),
            None => println!("Cannot execute this statement"),
        }
    }
}
