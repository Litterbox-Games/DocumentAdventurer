// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rust_search::{FilterExt, SearchBuilder};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![file_stats])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn file_stats() -> Vec<String> {
  return SearchBuilder::default()
		.location("~/Desktop")
		.custom_filter(|dir| dir.metadata().unwrap().is_file())
		.custom_filter(|dir| !dir.metadata().unwrap().permissions().readonly())
		.build()
		.collect();
}