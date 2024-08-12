use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    flag: String,
}

fn print_todo(todo: &Vec<String>) {
    println!("The Todo List:");
    println!("{:?}", todo);
}

fn add_todo(todo: &mut Vec<String>) {
    println!("Please enter the todo to add:");
    let mut task = String::new();
    io::stdin()
        .read_line(&mut task)
        .expect("something went wrong");

    todo.push(task.trim().to_string()); // Add the task to the todo list
    print_todo(todo); // Print the updated todo list
}

fn remove_todo(todo: &mut Vec<String>, index: usize) {
    if index < todo.len() {
        todo.remove(index); // Remove the item at the specified index
    } else {
        println!("Index out of bounds");
    }
    print_todo(todo);
}

fn read_store(path: &Path, todo: &mut Vec<String>) {
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
            todo.extend(s.lines().map(|line| line.to_string()));
        },
    }
}

fn save_store(path: &Path, todo: &mut Vec<String>) {
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    for task in todo {
        if let Err(why) = writeln!(file, "{}", task) {
            panic!("couldn't write to {}: {}", display, why);
        }
    }
}

fn main() {
    println!("!!Welcome User!!");
    let args = Cli::parse();

    let path = Path::new("src/store.txt");

    println!("flag: {:?}", args.flag);
    let mut todo: Vec<String> = Vec::new();

    read_store(path, &mut todo);

    match args.flag.as_str() {
        "show-todo" => print_todo(&todo),
        "add-todo" => add_todo(&mut todo),
        "remove-todo" => {
            println!("Please enter the index of the todo to remove:");
            let mut index_input = String::new();
            io::stdin().read_line(&mut index_input)
                .expect("Failed to read line");

            let index: usize = match index_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid index input");
                    return;
                }
            };

            remove_todo(&mut todo, index);
        },
        _ => println!("Something went wrong"),
    }
    save_store(path, &mut todo)
}
