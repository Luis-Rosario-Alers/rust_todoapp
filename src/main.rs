mod structs;
mod helpers;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::process;
use crate::structs::{TodoItem, TodoLists};

fn main() {
    program_start();
    let mut todo_lists = initialize_lists().unwrap();
    loop {
        let mut command: String = String::new();
        
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        
        let command = command.trim();
        
        handle_command(&command, &mut todo_lists);
    }
}

fn initialize_lists() -> Result<TodoLists, serde_json::Error> {
    let file = match File::open("data.json") {
        Ok(file) => file,
        Err(error) => panic!("Error opening file: {:?}", error),
    };
    let reader = BufReader::new(file);
    
    let lists: Result<TodoLists, _> = serde_json::from_reader(reader);
    lists
}
fn program_start() {
    println!("Welcome to the todo list program.");
    println!("Type 'help' to see a list of commands.");
}

fn handle_command(command: &str, todo_lists: &mut TodoLists) {
    match command {
        "help" => {
            println!("help"); // show help screen
        },
        "add" => { 
            todo_lists.get_active_list().add_item(); // add a new todo item to current list
        },
        "quit" => {
            process::exit(0); // quit program
        }
        "switch" => {
            todo_lists.switch_active_list() // switch to another list
        }
        _ => {
            println!("Unknown command.");
        }
    }
}


fn show_help() {
    println!("Available commands:");
    println!("  help - Show this help message");
    println!("  add - Add a new todo item");
    println!("  quit - Quit the program");
    println!("  switch - Switch to another list");
}
