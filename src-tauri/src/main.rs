// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetState {
    pub mood: String,
    pub energy: f32,
    pub happiness: f32,
}

impl Default for PetState {
    fn default() -> Self {
        Self {
            mood: "happy".into(),
            energy: 80.0,
            happiness: 70.0,
        }
    }
}

pub struct AppState(pub Mutex<PetState>);

#[tauri::command]
fn get_pet_state(state: tauri::State<'_, AppState>) -> PetState {
    state.0.lock().unwrap().clone()
}

#[tauri::command]
fn pet_interact(action: String, state: tauri::State<'_, AppState>) -> PetState {
    let mut pet = state.0.lock().unwrap();

    match action.as_str() {
        "pet" => {
            pet.happiness = (pet.happiness + 10.0).min(100.0);
        }
        "feed" => {
            pet.energy = (pet.energy + 15.0).min(100.0);
            pet.happiness = (pet.happiness + 5.0).min(100.0);
        }
        "sleep" => {
            pet.energy = (pet.energy + 30.0).min(100.0);
        }
        _ => {}
    }

    pet.mood = calculate_mood(pet.energy, pet.happiness);
    pet.clone()
}

#[tauri::command]
fn tick_pet(state: tauri::State<'_, AppState>) -> PetState {
    let mut pet = state.0.lock().unwrap();

    pet.energy = (pet.energy - 0.05).max(0.0);
    pet.happiness = (pet.happiness - 0.03).max(0.0);
    pet.mood = calculate_mood(pet.energy, pet.happiness);

    pet.clone()
}

fn calculate_mood(energy: f32, happiness: f32) -> String {
    if energy < 20.0 {
        "sleepy".into()
    } else if happiness > 70.0 {
        "happy".into()
    } else if happiness < 30.0 {
        "sad".into()
    } else {
        "neutral".into()
    }
}

fn main() {
    deskpet_lib::run()
}
