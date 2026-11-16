use walkdir::WalkDir;
use std::io::{self, Write};

fn prompt(message: &str) -> String { // Resueable for user input throuhgt the program without having to rewrite the code!
    print!("{}", message); 
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}



fn main() {
    let entry_input = prompt("Enter the path to the directory you want to search: ");
    let find_this = prompt("Enter the file or folder you wish to find: ");

    let found = WalkDir::new(&entry_input).into_iter().filter_map(Result::ok).find(|entry| *entry.file_name() == *find_this);
    if let Some(entry) = found {
        println!("Found: {}", entry.path().display());
    }
    else {
        println!("Not Found: {}", find_this);
    }
}




