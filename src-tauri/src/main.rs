#![cfg_attr(proc_macro_hygiene, decl_macro, not(debug_assertions), windows_subsystem = "windows")]
extern crate rocket;
extern crate rocket_contrib;

use rocket::http::Status;
use rocket::response::Responder;
use rocket::{get, routes, Build, Request, Rocket, fairing::AdHoc};
use rocket::serde::{Serialize, Deserialize};
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

#[get("/todos")]
fn get_todos(_request: &Request) -> Json<Vec<Todo>> {
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
    rocket::build()
        .mount("/", routes![get_todos])
        .attach(AdHoc::on_ignite("Launch Rocket", |rocket| async {
            // Additional setup on Rocket launch if needed
            Ok(rocket)
        }))
}

fn main() {
    // Initialize and run Tauri
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}