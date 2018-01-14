use std::io::{self, Write};


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

fn main() {
    loop {
        print_prompt();
        let input_buffer = read_input();

        match input_buffer.trim().as_ref() {
            ".exit" => std::process::exit(0),
            _ => println!("Cannot process this input"),
        }
    }
}
