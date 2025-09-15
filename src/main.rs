use clap::{Parser};
use rusqlite::Result;

pub mod db;
pub mod models;
// show help options when base executable is called (no flags)
//
// Checklist / roadmap
// [] (pre) briefly review rust book and relevant guides
//      - devtooling
//      - testing
//      - documents
// [] access sqlite database via rust runtime
// let conn = Connection::open("my_database.sqlite3");
// use `Connection::open_in_memory()` if you prefer an in-memory DB
// Ok(())
//
// [] create data model for todo items
// [] create a function for adding a todo to the database
// [] create a function for removing a todo from the database
// [] create a function for updating a todo from the database
// [] create a function for getting the todos from the database
// [] ensure the above functions are executable via commandline
//    ex: ./todo-rs get --id
//    ex: ./todo-rs list
//    ex: ./todo-rs update --id 1 --title foo --description bar
//    ex: ./todo-rs add --title foo --description bar (have some default values)
//    ex: ./todo-rs delete --id 1
//


/* #[derive(Clone, ValueEnum)]
enum Actions {
    Add,
    Update,
    Remove,
    Get,
} */

#[derive(Clone, Debug,clap::ValueEnum)]
enum Actions { Add, Update, Remove, Get }

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    times: u8,

    #[arg(short, long,value_enum)]
    action: Actions,

    #[arg(short, long)]
    description: Option<String>,

    #[arg(long)]
    title: Option<String>,
}

#[allow(dead_code)]
fn main() -> Result<()> {
    println!("Starting up Rust todo CLI");

    let args = Args::parse();

    let conn = db::initialize_database().expect("failed to initialize database");

    match args.action {
        Actions::Add => {
            println!("Adding a todo!");
            if let Some(title) = args.title {
                match db::add_todo(&conn, &title) {
                    Ok(_) => println!("Added todo: {}", title),
                    Err(e) => println!("Error adding todo: {}", e),
                }
            } else {
                println!("The --title argument is required for the 'add' action");
            }
        }
        Actions::Remove => {
            println!("Removing a todo");
            if let Some(title) = args.title {
                match db::remove_todo(&conn, &title) {
                    Ok(_) => println!("Removed todo: {}", title),
                    Err(e) => println!("Error removing todo: {}", e),
                }
            }
        }
        Actions::Update=> println!("Updating a todo"),
        Actions::Get => println!("Listing all todos"),
    }

    Ok(())
}
