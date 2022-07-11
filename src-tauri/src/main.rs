#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod todo;
use todo::{Todo, TodoApp};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_todos, new_todo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_todos() -> Vec<Todo> {
    let app: TodoApp = TodoApp::new().unwrap();
    let todos: Vec<Todo> = app.get_todos().unwrap();
    match app.conn.close() {
        Ok(o) => o,
        Err(e) => panic!("{:?}", e),
    };
    todos
}

#[tauri::command]
fn new_todo(todo: Todo) -> bool {
    let app: TodoApp = TodoApp::new().unwrap();
    let result = app.new_todo(todo);
    match app.conn.close() {
        Ok(o) => o,
        Err(e) => panic!("{:?}", e),
    };
    result
}
