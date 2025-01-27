// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{array::from_ref, collections::HashMap};

use database::db;

mod database;


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_exercise, create_exercise,get_every_exercise,
      get_every_workout, get_workout,get_workout_exercises
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn save_exercise(name:&str, classes:&str) {
  println!("Saving exercides {} with classees {}", name, classes);
}

#[tauri::command]
fn get_exercise(name:&str) -> database::Exercise {
  let db = database::db::new();
  return db.get_exercise(name)
}

#[tauri::command]
fn get_workout(name:&str) -> database::Workout {
  let db = database::db::new();
  return db.get_workout(name)
}

#[tauri::command]
fn get_every_workout() -> Vec<database::Workout> {
  let db = database::db::new();
  return db.get_every_workout();
}


#[tauri::command]
fn get_every_exercise() -> Vec<database::Exercise> {
  let db = database::db::new();
  return db.get_every_exercise();
}

#[tauri::command]
fn get_workout_exercises(name: &str) -> Vec<database::Exercise> {
  let db = database::db::new();
  return db.get_workout_exercises(name);
}



#[tauri::command]
fn create_exercise(name: &str, description:String, steps:Vec<String>) {
  let mut db = database::db::new();

  let e_name: String = name.to_string();

  let exercise: database::Exercise = database::Exercise {
    description,
    steps,
    name:e_name
  };
  db.create_exercise(name.to_string(),exercise);
}