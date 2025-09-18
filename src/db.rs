use rusqlite::{Connection, Result};
use rusqlite_migration::MigrationsBuilder;
use include_dir::{Dir, include_dir};

use crate::models::Todo;

static MIGRATION_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");

pub fn initialize_database() -> Result<Connection> {
    let mut conn = Connection::open("my_database.sqlite3")?;

    let migrations: rusqlite_migration::Migrations = MigrationsBuilder::from_directory(&MIGRATION_DIR).unwrap().finalize();
    migrations.to_latest(&mut conn).unwrap();

    Ok(conn)
}

pub fn add_todo(conn: &Connection, item: &str) -> Result<usize> {
    conn.execute("INSERT INTO todos (item) VALUES (?1)", [item])
}

pub fn remove_todo(conn: &Connection, item: &str) -> Result<usize> {
    conn.execute("DELETE FROM todos WHERE item=(?1)", [item])
}

pub fn update_todo(conn: &Connection, item: &Todo) -> Result<usize> {
    conn.execute("UPDATE todos SET item=(?1), description=(?2) WHERE id=(?3)", &[&item.item, &item.description, &item.id.to_string()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_todo() {
        let mut conn = Connection::open_in_memory().unwrap();
        let migrations: rusqlite_migration::Migrations = MigrationsBuilder::from_directory(&MIGRATION_DIR).unwrap().finalize();
        migrations.to_latest(&mut conn).unwrap();

        let result = add_todo(&conn, "test todo");
        assert!(result.is_ok());

        let mut stmt = conn
            .prepare("SELECT item FROM todos WHERE item='test todo'")
            .unwrap();
        let mut rows = stmt.query([]).unwrap();
        let row = rows.next().unwrap();
        assert!(row.is_some());
    }
}
