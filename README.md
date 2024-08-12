# CLI Todo List
A simple command-line interface (CLI) todo list application built in Rust. This project allows you to manage a list of tasks with persistence, meaning your todos will be saved to a file and available every time you run the program.

## Features
- View Todos: Display the current list of tasks.
- Add Todos: Add new tasks to your todo list.
- Remove Todos: Remove tasks from your todo list by specifying their index.
- Persistence: The todo list is saved to a file (store.txt) and is automatically loaded when the application starts.
## Usage
### Running the Program
Clone the repository and navigate to the project directory.

Build and run the project using Cargo:
```bash
cargo run -- <flag>
```
### Available Flags
- show-todo: Displays the current todo list.
- add-todo: Prompts you to add a new task to the todo list.
- remove-todo: Prompts you to remove a task from the todo list by specifying its index.
Example Commands
```bash
# Display the current todo list
cargo run -- show-todo

# Add a new task to the todo list
cargo run -- add-todo

# Remove a task by its index
cargo run -- remove-todo
```
### File Persistence
The todo list is saved in a file called store.txt located in the src/ directory. This file is automatically read when the application starts, and any changes to the todo list are saved back to this file.

### Dependencies
Clap: A command-line argument parser for Rust, used for handling the CLI flags.
Contributing
Feel free to fork this repository, submit issues, or make pull requests if you want to contribute or suggest improvements.