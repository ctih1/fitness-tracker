use std::{collections::HashMap, fs::{read_to_string, File}, io::{BufWriter, Write}};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
enum Theme {
    Light,
    Dark
}


#[derive(Deserialize, Serialize, Clone)]
struct WorkoutHistory {
    exercise: Exercise,
    reps: i32,
    weight: i32,
    date: i32 // epoch timestamp
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Exercise {
    pub description: String,
    pub steps: Vec<String>,
    pub name: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Workout { // technically just a set of exercises 
    pub exercises: Vec<String>, // a map of workout and the exercises,
    pub name:String, 
    pub description: String
}

#[derive(Deserialize, Serialize, Clone)]
struct Data {
    theme: Theme,
    exercises: HashMap<String,Exercise>,
    workouts: HashMap<String,Workout>,
    history: Vec<WorkoutHistory>
}

pub struct db {
    data: Data
}

impl db {
    pub fn new() -> Self {
        println!("{}",std::env::current_dir().unwrap().into_os_string().into_string().unwrap());
        File::create("/data/db.json");
        let file_content:Result<String, std::io::Error>  = read_to_string("./data/db.json");
        if file_content.is_err() {
            eprintln!("Failed to load database {:?}", file_content.as_ref().unwrap_err());
        }
        let mut data: Data = serde_json::from_str(file_content.unwrap().as_str()).expect("Failed to load database!");

        return Self { data }

    }

    pub fn create_exercise(&mut self,name: String, exercise: Exercise) {
        println!("Creating thing"); 
        let file: Result<File, std::io::Error> = File::create("./data/db.json");
        if file.is_err() {
            eprint!("Failed to load database file!");
            panic!("HELP ME");
        }
        let mut writer: BufWriter<File> = BufWriter::new(file.unwrap());
        self.data.exercises.insert(name,exercise);

        let result: Result<(), serde_json::Error> = serde_json::to_writer(&mut writer, &self.data);
        if result.is_err() {
            eprint!("Failed to write database {:?}", result.as_ref().unwrap_err());
            panic!("HELP ME AGAIN")
        }
        let result: Result<(), std::io::Error> = writer.flush();
        if result.is_err() {
            eprint!("Failed to write database {:?}", result.as_ref().unwrap_err());
        } if result.is_ok() {
            println!("Succeeded");

        }
    }
    
    pub fn get_every_exercise(&self) -> Vec<Exercise> {
        return self.data.exercises.values().cloned().collect::<Vec<Exercise>>();
    }

    pub fn get_every_workout(&self) -> Vec<Workout> {
        return self.data.workouts.values().cloned().collect::<Vec<Workout>>();
    }
    
    pub fn get_exercise(&self, name: &str) -> Exercise {
        return self.data.exercises.get(name).cloned().unwrap();
    }

    pub fn get_workout_exercises(&self, name:&str) -> Vec<Exercise> {
        let mut exercises: Vec<Exercise> = vec![];
        for exercise in self.data.workouts.get(name).cloned().unwrap().exercises.iter() {
            exercises.push(self.get_exercise(&exercise));
        }
        return exercises;
    }

    pub fn get_workout(&self, name:&str) -> Workout {
        println!("{:?}", self.data.workouts.keys());
        println!("{}", name);
        return self.data.workouts.get(name).cloned().unwrap();
    }
}
