use std::io;
use std::io::Write;

pub fn read_input(console_text: &str) -> String {
    print!("{}", console_text);
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    input
}