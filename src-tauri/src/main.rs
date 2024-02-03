// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(proc_macro_hygiene, decl_macro, not(debug_assertions), windows_subsystem = "windows")]
#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;
use rocket::{Rocket, Build, serde::{Deserialize, Serialize}};
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

#[get("/todos")]
fn get_todos() -> Json<Vec<Todo>> {
    Json(vec![
        Todo {
            id: 1,
            title: "Learn Rust".to_string(),
            completed: false,
        },
        Todo {
            id: 2,
            title: "Build a Rocket app".to_string(),
            completed: false,
        },
    ])
}

fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![get_todos])
}

fn main() {
    rocket().launch();
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}