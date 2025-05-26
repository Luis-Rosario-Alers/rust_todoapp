use std::fs::File;
use rand::Rng;
use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};
use std::io::Write;

#[derive(Parser, Debug)]
#[command(name = "")]
pub struct TodoCommand {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new todo item
    Add {
        /// Title of the todo item
        title: String,
        /// Description of the todo item (optional)
        #[arg(short = 'd', long = "description")]
        description: Option<String>,
    },
    /// List all items
    List {
        /// Show completed items
        #[arg(short = 'v', long = "verbose")]
        completed: bool,
    },
    /// Complete an item by its index (starting from 1)
    Complete {
        /// Item index (starting from 1)
        index: usize,
    },
    /// Remove current list 
    RemoveList,
    /// Remove list item
    Remove {
        /// Item index (starting from 1)
        index: usize,
    },
    /// Create a new todo list
    Create {
        /// Name of new todo list
        name: String,
    },
    /// Switch lists
    Switch,
    /// Rename the current list
    RenameList {
        /// New name for the list
        name: String,
    },
    
    /// Rename a todo item
    RenameItem {
        /// Item index (starting from 1)
        index: usize,
        /// New title for the item
        title: String,
    },
    
    /// Change an item's description
    EditItem {
        /// Item index (starting from 1)
        index: usize,
        /// New description for the item
        description: String,
    },
    
    /// Quit the program
    Quit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    title: String,
    description: String,
    completed: bool,
    id: i32,
}

impl TodoItem {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            completed: false,
            id: 0,
        }
    }
    
    pub fn new_with(title: String, description: String) -> Self {
        let mut rng = rand::rng();
        Self {
            title,
            description,
            completed: false,
            id: rng.random_range(0..100000)
        }
    }
    pub fn title(&self) -> &String {
        &self.title
    }
    pub fn description(&self) -> &String {
        &self.description
    }
    pub fn completed(&self) -> bool {
        self.completed
    }
    pub fn id(&self) -> i32 {
        self.id
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    name: String,
    items: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            items: Vec::new(),
        }
    }

    pub fn new_with_name(name: String) -> Self {
        Self {
            name,
            items: Vec::new(),
        }
    }

    pub fn new_with(name: String, items: Vec<TodoItem>) -> Self {
        Self {
            name,
            items,
        }
    }

    pub fn add_item(&mut self, title: String, description: String) {
        let todo_item: TodoItem = TodoItem::new_with(title, description);

        println!("Added todo item: {:?}", todo_item);

        self.items.push(todo_item);
    }

    pub fn display_items_compact(&self) {
        // Items should be displayed like this
        // 1. Do laundry [Status: 🔴]
        for (i, item) in self.items.iter().enumerate() {
            println!("{}. {} [Status: {}]", i+1, item.title, if item.completed { "🟢" } else { "🔴" });
        }
    }

    pub fn display_items_verbose(&self) {
        // Items should be displayed like this

        // 1. Do laundry [Status: 🔴]
        //      I need to do laundry for my mom today.

        for (i, item) in self.items.iter().enumerate() {
            println!("{}. {} [Status: {}]\n     {}", i+1, item.title, if item.completed { "🟢" } else { "🔴" }, item.description);
        }
    }

    pub fn display_items(&self, completed: bool) {
        if self.items.is_empty() {
            println!("No items in the list.");
            return;
        }

        if completed {
            self.display_items_verbose();
        } else {
            self.display_items_compact();
        }
    }

    pub fn complete_item(&mut self, index: usize) {
        if index > 0 && index <= self.items.len() {
            let item = &mut self.items[index - 1];
            item.completed = true;
            println!("Completed item: {}", item.title);
        } else {
            println!("Invalid index: {}", index);
        }
    }

    pub fn remove_item(&mut self, index: usize) {
        if index > 0 && index <= self.items.len() {
            let item = &self.items[index - 1];
            println!("Removing item: {}", item.title);
            self.items.remove(index - 1);
        } else {
            println!("Invalid index: {}", index);
        }
    }

    pub fn change_list_name(&mut self, name: String) {
        println!("Changed list name from '{}' to '{}'", self.name, name);
        self.name = name;
    }

    pub fn change_item_name(&mut self, index: usize, name: String) {
        if index > 0 && index <= self.items.len() {
            let item= &mut self.items[index - 1];
            item.title = name;
        } else {
            println!("Invalid index: {}", index);
        }
    }

    pub fn change_item_description(&mut self, index: usize, description: String) {
        if index > 0 && index <= self.items.len() {
            let item = &mut self.items[index - 1];
            item.description = description;
        } else {
            println!("Invalid index: {}", index);
        }  
    }



    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn items(&self) -> &Vec<TodoItem> {
        &self.items
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoLists {
    pub lists: Vec<TodoList>,
    pub active_index: usize,
}


impl TodoLists {
    pub fn new() -> Self {
        Self {
            lists: vec![TodoList::new()],
            active_index: 0,
        }
    }

    pub fn new_with(lists: Vec<TodoList>) -> Self {
        Self {
            lists,
            active_index: 0,
        }
    }

    pub fn get_active_list(&mut self) -> &mut TodoList {
        self.save_state();
        &mut self.lists[self.active_index]
    }

    pub fn switch_active_list(&mut self) {
        if self.active_index < self.lists.len() - 1 {
            self.active_index += 1;
        } else {
            self.active_index = 0;
        }
        println!("Switched to list: {}", self.active_index);
    }

    fn save_state(&self) {
        let mut file = match File::create("data.json") {
            Ok(file) => file,
            Err(error) => panic!("Error opening file: {:?}", error),
        };
        let serialized_json = serde_json::to_string(self);
        match serialized_json {
            Ok(json) => {
                file.write_all(json.as_bytes()).expect("Failed to write to file");
            },
            Err(error) => panic!("Error serializing json: {:?}", error),
        }
    }

    pub fn create_new_list(&mut self, name: String) {
        let new_list = TodoList::new_with_name(name);
        println!("Created new list: {}", new_list.name());
        self.lists.push(new_list);
        self.active_index = self.lists.len() - 1;
        self.save_state();
    }
    
    pub fn remove_list(&mut self) {
        match self.lists.len() {
            1 => println!("There is only one list, cannot remove it."),
            _ => println!("Removed list: {}", self.lists[self.active_index].name()),
        }
        
        if self.lists.len() > 1 {
            self.lists.remove(self.active_index);
            if self.active_index > 0 { self.active_index -= 1 } else { self.active_index = 0 };
            self.save_state();
        }
    }
}
