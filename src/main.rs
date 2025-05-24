mod structs;
mod helpers;

use std::fs::File;
use std::io;
use std::io::{BufReader, Write};
use std::process;
use crate::structs::{TodoLists, TodoCommand};
use clap::{Command, Parser};
use shlex::split;
use structs::Commands;

fn main() {
    program_start();
    let mut todo_lists = initialize_lists().unwrap();
    loop {
        print!("todo> ");
        io::stdout().flush().unwrap();
        let mut command: String = String::new();
        
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let command = command.trim();
        if command.is_empty() {
            continue;
        }

        if command == "quit" || command == "q" {
            break;
        }

        let args: Vec<String> = match shlex::split(command) {
            Some(args) => args,
            None => {
                println!("Invalid command syntax.");
                continue;
            }
        };

        let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

        match TodoCommand::try_parse_from(std::iter::once("todo").chain(args_refs)) {
            Ok(cmd) => handle_command(cmd.command, &mut todo_lists),
            Err(err) => {
                println!("{}", err);
            }
        }        
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

fn handle_command(command: Commands, todo_lists: &mut TodoLists) {
    // TODO: at some point i need to add actual CLI parsing for commands
    match command {
        Commands::Add { title, description } => {
            todo_lists.get_active_list().add_item(title, description); // add a new todo item to current list
        },
        Commands::Quit => {
            process::exit(0); // quit program
        },
        Commands::Switch => {
            todo_lists.switch_active_list(); // switch to another list
        },
        Commands::List { completed } => {
            todo_lists.get_active_list().display_items(completed);
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
