mod structs;

use std::fs::File;
use std::io;
use std::io::{BufReader, Write};
use std::process;
use crate::structs::{TodoLists, TodoCommand};
use clap::{Parser};
use structs::Commands;

fn main() {
    program_start();
    let mut todo_lists = match initialize_lists() {
        Ok(lists) => lists,
        Err(e) => {
            eprintln!("Error initializing todo lists: {}", e);
            TodoLists::new()
        }
    };

    loop {
        print!("todo/{}> ", todo_lists.get_active_list().name());
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
    match command {
        Commands::Add { title, description } => {
            match description {
                Some(desc) => todo_lists.get_active_list().add_item(title, desc),
                None => todo_lists.get_active_list().add_item(title, "".to_string()),
            }
        },
        Commands::Quit => {
            process::exit(0); 
        },
        Commands::Switch => {
            todo_lists.switch_active_list(); 
        },
        Commands::List { completed } => {
            todo_lists.get_active_list().display_items(completed);
        },
        Commands::Complete { index } => {
            todo_lists.get_active_list().complete_item(index);
        },
        Commands::Remove { index } => {
            todo_lists.get_active_list().remove_item(index);
        },
        Commands::Create { name } => {
            todo_lists.create_new_list(name);
        }
        Commands::Change { name, item} => {
            match item {
                Some(item) => todo_lists.get_active_list().change_item_name(name, item),
                None => todo_lists.get_active_list().change_list_name(name),
            }
        }
    }
}