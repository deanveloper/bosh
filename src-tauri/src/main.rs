#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use bosh_rs::rider::Entity;
use bosh_rs::{Line, Track};
use once_cell::sync::Lazy;
use tauri::command;

use crate::serialization::boshtf::{BoshTFEntity, BoshTFTrack};

mod serialization;
mod track_loading;

static TRACK: Lazy<Mutex<Track>> = Lazy::new(|| Mutex::new(Track::new(vec![], vec![])));

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            clear,
            add_line,
            remove_line,
            add_entity,
            remove_entity,
            entity_positions_at,
            load_track,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[command]
fn add_line(line: Line) -> Result<Vec<Line>, String> {
    let mut track = TRACK.lock().map_err(|err| err.to_string())?;

    track.add_line(line);

    Ok(track.all_lines().clone())
}

#[command]
fn remove_line(line: Line) -> Result<Vec<Line>, String> {
    let mut track = TRACK.lock().map_err(|err| err.to_string())?;

    track.remove_line(&line);

    Ok(track.all_lines().clone())
}

#[command]
fn add_entity(entity: BoshTFEntity) -> Result<(), String> {
    let entity: Entity = (&entity).into();
    let mut track = TRACK.lock().map_err(|err| err.to_string())?;

    track.create_entity(entity);

    Ok(())
}

#[command]
fn remove_entity(entity: BoshTFEntity) -> Result<(), String> {
    let entity: Entity = (&entity).into();
    let mut track = TRACK.lock().map_err(|err| err.to_string())?;

    track.remove_entity(entity);

    Ok(())
}

#[command]
fn entity_positions_at(frame: usize) -> Result<Vec<Entity>, String> {
    let serialized_positions = TRACK
        .lock()
        .map_err(|err| {
            eprintln!("{}", err);
            err.to_string()
        })?
        .entity_positions_at(frame);

    Ok(serialized_positions)
}

#[command]
fn load_track(path: String) -> Result<BoshTFTrack, String> {
    let track = track_loading::load(&path).map_err(|err| {
        eprintln!("{:#}", err);
        err.to_string()
    })?;
    *TRACK.lock().map_err(|err| err.to_string())? = (&track).into();

    Ok(track)
}

#[command]
fn clear() {
    let mut track = TRACK.lock().unwrap();

    *track = Track::new(vec![], vec![]);
}
