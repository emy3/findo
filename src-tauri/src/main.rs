// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(proc_macro_hygiene, decl_macro, not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}