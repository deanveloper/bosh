#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::ops::Deref;
use std::sync::Mutex;

use bosh_rs;
use bosh_rs::{Line, Track};
use bosh_rs::rider::Entity;
use once_cell::sync::Lazy;
use tauri::command;

use crate::serialized_types::SerializableEntity;

mod serialized_types;

static TRACK: Lazy<Mutex<Track>> = Lazy::new(|| Mutex::new(Track::new(
    &[],
    &vec![],
)));

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![clear, add_line, add_entity, entity_positions_at])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[command]
fn add_line(line: Line) -> Result<Vec<Line>, String> {
    let mut track = TRACK.deref().lock().unwrap();

    track.add_line(line);

    Ok(track.all_lines().clone())
}

#[command]
fn remove_line(line: Line) -> Result<Vec<Line>, String> {
    let mut track = TRACK.deref().lock().unwrap();

    track.remove_line(line);

    Ok(track.all_lines().clone())
}

#[command]
fn add_entity(js_entity: SerializableEntity) -> Result<(), String> {
    let mut track = TRACK.deref().lock().unwrap();

    let entity: Result<Entity, anyhow::Error> = js_entity.into();
    track.create_entity(entity.map_err(|err| err.to_string())?);

    Ok(())
}

#[command]
fn remove_entity(js_entity: SerializableEntity) -> Result<(), String> {
    let mut track = TRACK.deref().lock().unwrap();

    let entity: Result<Entity, anyhow::Error> = js_entity.into();
    track.remove_entity(entity.map_err(|err| err.to_string())?).ok_or(|| "entity not found".to_owned())
}

#[command]
fn entity_positions_at(frame: u64) -> Vec<SerializableEntity> {
    let serialized_positions = TRACK.deref().lock().unwrap().rider_positions_at(frame as usize)
        .into_iter()
        .map(|entity| SerializableEntity::new(&entity))
        .collect();

    serialized_positions
}

#[command]
fn clear() {
    let mut track = TRACK.deref().lock().unwrap();

    *track = Track::new(&[], &vec![]);
}
