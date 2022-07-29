use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub label: String,
    pub done: bool,
    pub is_delete: bool,
}

pub struct TodoApp {
    pub conn: Connection,
}

impl TodoApp {
    pub fn new() -> Result<TodoApp> {
        let db_path = env::var("DB_PATH").unwrap();
        let conn = Connection::open(db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS Todo (
                id          varchar(64)     PRIMARY KEY,
                label       text            NOT NULL,
                done        numeric         DEFAULT 0,
                is_delete   numeric         DEFAULT 0
            )",
            [],
        )?;
        Ok(TodoApp { conn })
    }

    pub fn get_todos(&self) -> Result<Vec<Todo>> {
        let mut stmt = self.conn.prepare("SELECT * FROM Todo").unwrap();
        let todos_iter = stmt.query_map([], |row| {
            let done = row.get::<usize, i32>(2).unwrap() == 1;
            let is_delete = row.get::<usize, i32>(3).unwrap() == 1;

            Ok(Todo {
                id: row.get(0)?,
                label: row.get(1)?,
                done,
                is_delete,
            })
        })?;
        let mut todos: Vec<Todo> = Vec::new();

        for todo in todos_iter {
            todos.push(todo?);
        }

        Ok(todos)
    }

    pub fn new_todo(&self, todo: Todo) -> bool {
        let Todo { id, label, .. } = todo;
        match self
            .conn
            .execute("INSERT INTO Todo (id, label) VALUES (?, ?)", [id, label])
        {
            Ok(insert) => {
                println!("{} row inserted", insert);
                true
            }
            Err(err) => {
                println!("some error: {}", err);
                false
            }
        }
    }
}
