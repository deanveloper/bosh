use std::fs::File;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use read_from::ReadFrom;

use crate::serialization;
use crate::serialization::boshtf::BoshTFTrack;
use crate::serialization::lrcom::LRComTrack;

pub fn load(file_path: &str) -> Result<BoshTFTrack> {
    // first to parse based on its extension
    let extension = get_extension(PathBuf::from(file_path));

    let file =
        File::open(file_path).with_context(|| format!("error while opening file {}", file_path))?;

    // todo - do not rely on extension
    match extension.as_deref() {
        Some("boshtf") => deserialize_boshtf(file),
        Some("track.json") => deserialize_lrcom(file),
        Some("trk") => deserialize_lra(file),
        Some(other) => Err(anyhow!("not a recognizable extension: {}", other)),
        None => Err(anyhow!("needs an extension: {}", file_path)),
    }
}

fn deserialize_boshtf(f: File) -> Result<BoshTFTrack> {
    serde_json::from_reader(f).context("error while parsing file as boshtf format")
}

fn deserialize_lrcom(f: File) -> Result<BoshTFTrack> {
    let track: LRComTrack =
        serde_json::from_reader(f).context("error while parsing file as lr.com format")?;

    BoshTFTrack::try_from(&track).context("error converting lr.com to local format")
}

fn deserialize_lra(f: File) -> Result<BoshTFTrack> {
    let trk = serialization::trk::TrkTrack::read_from(f)
        .context("error while parsing file as lr-a format")?;

    Ok(BoshTFTrack::from(&trk))
}

fn get_extension(mut path: PathBuf) -> Option<String> {
    let mut full_extension: String = String::with_capacity(10);

    full_extension.push_str(path.extension()?.to_str()?);
    path.set_extension("");

    // recursively get extensions
    if let Some(ext) = get_extension(path) {
        full_extension.insert_str(0, &format!("{}.", &ext));
    }

    Some(full_extension)
}
