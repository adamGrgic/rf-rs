use crate::models::{AddTodo, RemoveTodo, Todo, UpdateTodo};
use include_dir::{include_dir, Dir};
use rusqlite::{Connection, Result};
use rusqlite_migration::MigrationsBuilder;

static MIGRATION_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");

pub fn initialize_database() -> Result<Connection> {
    let mut conn = Connection::open("my_database.sqlite3")?;

    let migrations: rusqlite_migration::Migrations =
        MigrationsBuilder::from_directory(&MIGRATION_DIR)
            .unwrap()
            .finalize();
    migrations.to_latest(&mut conn).unwrap();

    Ok(conn)
}

pub fn add_todo(conn: &Connection, todo: &AddTodo) -> Result<usize> {
    conn.execute(
        "INSERT INTO todos (title, description) VALUES (?1, ?2)",
        &[&todo.title, &todo.description],
    )
}

pub fn remove_todo(conn: &Connection, todo: &RemoveTodo) -> Result<usize> {
    conn.execute("DELETE FROM todos WHERE id=(?1)", &[&todo.id.to_string()])
}

pub fn update_todo(conn: &Connection, todo: &UpdateTodo) -> Result<usize> {
    conn.execute(
        "UPDATE todos SET title=(?1), description=(?2) WHERE id=(?3)",
        &[
            &todo.title,
            &todo.description,
            &todo.id.to_string(),
        ],
    )
}

pub fn get_all_todos(conn: &Connection) -> Result<Vec<Todo>> {
    let mut stmt = conn.prepare("SELECT id, item, description FROM todos")?;
    let todo_iter = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            // completed: row.get(3)?,
            // Assuming created_at is stored as a string. Adjust if it's a different type.
            // created_at: row.get(4)?,
        })
    })?;

    let mut todos = Vec::new();
    for todo in todo_iter {
        todos.push(todo?);
    }
    Ok(todos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_todo() {
        let mut conn = Connection::open_in_memory().unwrap();
        let migrations: rusqlite_migration::Migrations =
            MigrationsBuilder::from_directory(&MIGRATION_DIR)
                .unwrap()
                .finalize();
        migrations.to_latest(&mut conn).unwrap();

        let add_todo_test = AddTodo {
            title: "test title".to_string(),
            description: "test description".to_string(),
        };

        let result = add_todo(&conn, &add_todo_test);
        assert!(result.is_ok());

        let mut stmt = conn
            .prepare("SELECT title, description FROM todos WHERE title='test title'")
            .unwrap();
        let mut rows = stmt.query([]).unwrap();
        let row = rows.next().unwrap();
        assert!(row.is_some());
    }
}
