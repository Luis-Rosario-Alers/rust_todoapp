use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::{helpers, structs};

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
        let title_name = helpers::read_input();
        let description_name = helpers::read_input();

        let todo_item: TodoItem = TodoItem::new_with(title_name, description_name);

        println!("Added todo item: {:?}", todo_item);
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
        &mut self.lists[self.active_index]
    }

    pub fn switch_active_list(&mut self) {
        if self.active_index < self.lists.len() - 1 {
            self.active_index += 1;
        } else {
            self.active_index = 0;
        }
    }

}