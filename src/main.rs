use clap::Parser;
use rusqlite::Result;

mod db;
mod models;

use crate::models::{AddTodo, RemoveTodo, UpdateTodo};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    Add {
        #[arg(long)]
        title: String,
        #[arg(long)]
        description: String,
    },
    Remove {
        #[arg(long)]
        id: i32,
    },
    Update {
        #[arg(long)]
        id: i32,
        #[arg(long)]
        title: String,
        #[arg(long)]
        description: String,
    },
    Get,
}

fn main() -> Result<()> {
    println!("Starting up Rust todo CLI");

    let cli = Cli::parse();
    let conn = db::initialize_database().expect("failed to initialize database");

    match cli.command {
        Commands::Add { title, description } => {
            println!("Adding a todo!");
            let add_todo = AddTodo {
                title,
                description,
            };

            match db::add_todo(&conn, &add_todo) {
                Ok(_) => println!("Added todo: {}", add_todo.title),
                Err(e) => eprintln!("Error adding todo: {}", e),
            }
        }
        Commands::Remove { id } => {
            println!("Removing a todo");
            let remove_todo = RemoveTodo { id };
            match db::remove_todo(&conn, &remove_todo) {
                Ok(_) => println!("Removed todo id: {}", id),
                Err(e) => eprintln!("Error removing todo: {}", e),
            }
        }
        Commands::Update {
            id,
            title,
            description,
        } => {
            println!("Updating a todo");
            let update_todo = UpdateTodo {
                id,
                title,
                description,
            };
            match db::update_todo(&conn, &update_todo) {
                Ok(_) => println!("Updated todo: {}", update_todo.title),
                Err(e) => eprintln!("Error updating todo: {}", e),
            }
        }
        Commands::Get => {
            println!("Fetching all todos...");
            match db::get_all_todos(&conn) {
                Ok(todos) => {
                    if todos.is_empty() {
                        println!("No todos found.");
                    } else {
                        println!("--- Todos ---");
                        for todo in todos {
                            println!("{:?}", todo); // Requires `#[derive(Debug)]` on the Todo struct
                        }
                        println!("-------------");
                    }
                }
                Err(e) => eprintln!("Error fetching todos: {}", e),
            }
        }
    }

    Ok(())
}
