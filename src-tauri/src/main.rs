#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::ops::Deref;
use std::sync::Mutex;

use bosh_rs;
use bosh_rs::{Line, Track};
use bosh_rs::rider::Entity;
use bosh_rs::serialization::boshtf::BoshTFEntity;
use once_cell::sync::Lazy;
use tauri::command;

static TRACK: Lazy<Mutex<Track>> = Lazy::new(|| Mutex::new(Track::new(
    vec![],
    vec![],
)));

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![clear, add_line, remove_line, add_entity, remove_entity, entity_positions_at])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[command]
fn add_line(line: Line) -> Result<Vec<Line>, String> {
    let mut track = TRACK.deref().lock().map_err(|err| err.to_string())?;

    track.add_line(line);

    Ok(track.all_lines().clone())
}

#[command]
fn remove_line(line: Line) -> Result<Vec<Line>, String> {
    let mut track = TRACK.deref().lock().map_err(|err| err.to_string())?;

    track.remove_line(&line);

    Ok(track.all_lines().clone())
}

#[command]
fn add_entity(entity: BoshTFEntity) -> Result<(), String> {
    let entity: Entity = (&entity).into();
    let mut track = TRACK.deref().lock().map_err(|err| err.to_string())?;

    track.create_entity(entity);

    Ok(())
}

#[command]
fn remove_entity(entity: BoshTFEntity) -> Result<(), String> {
    let entity: Entity = (&entity).into();
    let mut track = TRACK.deref().lock().map_err(|err| err.to_string())?;

    track.remove_entity(entity);

    Ok(())
}

#[command]
fn entity_positions_at(frame: usize) -> Vec<Entity> {
    let serialized_positions = TRACK.deref().lock().unwrap().entity_positions_at(frame);

    serialized_positions
}

#[command]
fn clear() {
    let mut track = TRACK.deref().lock().unwrap();

    *track = Track::new(vec![], vec![]);
}
