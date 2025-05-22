pub struct TodoItem {
    title: String,
    description: String,
    completed: bool,
    id: i32,
}

impl TodoItem {
    pub fn new(&self) -> TodoItem {
        TodoItem {
            title: String::new(),
            description: String::new(),
            completed: false,
            id: 0,
        }
    }
    
    pub fn new_with(&self, title: String, description: String, completed: bool, id: i32) -> TodoItem {
        TodoItem {
            title,
            description,
            completed,
            id,
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

// struct Command {
//     
// }
