use std::fs::File;
use crate::helpers;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::io::Write;

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

    pub fn new_with(name: String, items: Vec<TodoItem>) -> Self {
        Self {
            name,
            items,
        }
    }

    pub fn add_item(&mut self) {
        let title_name = helpers::read_input("Title: ");
        let description_name = helpers::read_input("Description: ");

        let todo_item: TodoItem = TodoItem::new_with(title_name, description_name);

        println!("Added todo item: {:?}", todo_item);

        self.items.push(todo_item);
    }

    pub fn display_items(&self) {
        // Items should be displayed like this

        // 1. Do laundry [Status: 🔴]
        //      I need to do laundry for my mom today.

        for (i, item) in self.items.iter().enumerate() {
            println!("{}. {} [Status: {}]\n     {}", i+1, item.title, if item.completed { "🟢" } else { "🔴" }, item.description);
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
            lists: Vec::new(),
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


}