mod structs;
use std::io;
use std::process;


fn main() {
    program_start();
    loop {
        let mut command: String = String::new();
        
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        
        let command = command.trim();
        
        handle_command(&command);
    }
}

fn program_start() {
    println!("Welcome to the todo list program.");
    println!("Type 'help' to see a list of commands.");
}

fn handle_command(command: &str) {
    match command {
        "help" => {
            println!("help");
        },
        "add" => {
            println!("add");
        },
        "quit" => {
            process::exit(0);
        }
        _ => {
            println!("Unknown command.");
        }
    }
}

// fn add_todo(TodoItem: structs::TodoItem) {
// 
// }