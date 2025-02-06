use database::db;
use tauri_plugin_notification::{NotificationExt, ScheduleInterval};
use std::{array::from_ref, collections::HashMap};
mod database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            println!("{}", app.package_info().version.to_string());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_exercise,
            create_exercise,
            get_every_exercise,
            get_workout,
            get_every_workout,
            get_workout_exercises,
            save_workout,
            create_workout,
            get_history
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}

#[tauri::command]
fn get_exercise(app_handle: tauri::AppHandle, name: &str) -> database::Exercise {
    let db = database::db::new(app_handle);
    return db.get_exercise(name);
}

#[tauri::command]
fn get_workout(app_handle: tauri::AppHandle, name: &str) -> database::Workout {
    let db = database::db::new(app_handle);
    return db.get_workout(name);
}

#[tauri::command]
fn get_every_workout(app_handle: tauri::AppHandle) -> Vec<database::Workout> {
    let db = database::db::new(app_handle);
    return db.get_every_workout();
}

#[tauri::command]
fn get_every_exercise(app_handle: tauri::AppHandle) -> Vec<database::Exercise> {
    let db = database::db::new(app_handle);
    return db.get_every_exercise();
}

#[tauri::command]
fn get_workout_exercises(app_handle: tauri::AppHandle, name: &str) -> Vec<database::Exercise> {
    let db = database::db::new(app_handle);
    return db.get_workout_exercises(name);
}

#[tauri::command]
fn create_exercise(
    app_handle: tauri::AppHandle,
    name: &str,
    description: String,
    steps: Vec<String>,
    tracking_type: String
) {
    let mut db = database::db::new(app_handle);

    let e_name: String = name.to_string();

    let exercise: database::Exercise = database::Exercise {
        description,
        steps,
        name: e_name,
        tracking_type
    };
    db.create_exercise(name.to_string(), exercise);
}

#[tauri::command]
fn create_workout(
    app_handle: tauri::AppHandle,
    name: &str,
    description: String,
    exercises: Vec<String>
) {
    let mut db = database::db::new(app_handle);

    let e_name: String = name.to_string();

    let workout: database::Workout = database::Workout {
        name: e_name,
        description,
        exercises,
    };
    db.create_workout(name.to_string(), workout);
}

#[tauri::command]
fn save_workout( // saves workout history
    app_handle: tauri::AppHandle,
    name: String,
    exercises: Vec<database::ExerciseResults>,
    date: u32,
) {
    let mut db = database::db::new(app_handle);

    db.save_workout(database::WorkoutHistory {
        name,
        exercises,
        date,
    });
}

#[tauri::command]
fn get_history(app_handle: tauri::AppHandle) -> HashMap<String, Vec<database::WorkoutHistory>> {
    let db = database::db::new(app_handle);
    println!("Getting history");
    return db.get_history();
}
