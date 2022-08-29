//! Module for deserializing linerider.com tracks, aka ".track.json"

use std::borrow::Borrow;

use anyhow::{anyhow, Context, Error};
use bosh_rs::{Line, Vector2D};
use serde::{Deserialize, Serialize};

use crate::serialization::boshtf::{BoshTFEntity, BoshTFLine, BoshTFLineType, BoshTFTrack};

type Result<T> = anyhow::Result<T>;

#[derive(Clone, Serialize, Deserialize)]
pub struct LRComTrack {
    #[serde(skip)]
    next_line_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u64>,
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio: Option<String>,
    #[serde(rename = "startPosition")]
    start_position: LRComVec2,
    #[serde(skip_serializing_if = "Option::is_none")]
    riders: Option<Vec<LRComEntity>>,

    #[serde(default)]
    lines: Option<Vec<LRComLine>>,
    #[serde(rename = "linesArray", default, skip_serializing)]
    lines_array: Option<Vec<LRComLineArray>>,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct LRComEntity {
    #[serde(rename = "startPosition")]
    start_position: LRComVec2,
    #[serde(rename = "startVelocity")]
    start_velocity: LRComVec2,
    remountable: u8, // ... why is this not a boolean? are there more values?
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct LRComVec2 {
    x: f64,
    y: f64,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct LRComLine {
    id: u64,
    #[serde(rename = "type")]
    line_type: LRComLineType,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    flipped: bool,
    #[serde(rename = "leftExtended")]
    left_extended: bool,
    #[serde(rename = "rightExtended")]
    right_extended: bool,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct LRComLineArray(LRComLineType, u64, f64, f64, f64, f64, u8, bool);

impl From<LRComLineArray> for LRComLine {
    fn from(why: LRComLineArray) -> LRComLine {
        LRComLine {
            line_type: why.0,
            id: why.1,
            x1: why.2,
            y1: why.3,
            x2: why.4,
            y2: why.5,
            right_extended: why.6 & 0b10 > 0,
            left_extended: why.6 & 0b1 > 0,
            flipped: why.7,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[repr(u8)]
#[serde(try_from = "u8", into = "u8")]
pub enum LRComLineType {
    Normal = 0,
    Accelerate = 1,
    Scenery = 2,
}

impl From<LRComLineType> for u8 {
    fn from(line_type: LRComLineType) -> Self {
        line_type as u8
    }
}

impl TryFrom<u8> for LRComLineType {
    type Error = Error;

    fn try_from(value: u8) -> Result<LRComLineType> {
        match value {
            0 => Ok(LRComLineType::Normal),
            1 => Ok(LRComLineType::Accelerate),
            2 => Ok(LRComLineType::Scenery),
            _ => Err(anyhow!("not a valid line type: {value}")),
        }
    }
}

impl From<&LRComVec2> for Vector2D {
    fn from(vector: &LRComVec2) -> Vector2D {
        let vector = vector.borrow();
        Vector2D(vector.x, vector.y)
    }
}

impl From<&Vector2D> for LRComVec2 {
    fn from(vector: &Vector2D) -> LRComVec2 {
        let vector = vector.borrow();
        LRComVec2 {
            x: vector.0,
            y: vector.1,
        }
    }
}

impl TryFrom<&BoshTFLineType> for LRComLineType {
    type Error = Error;

    fn try_from(line_type: &BoshTFLineType) -> Result<LRComLineType> {
        match line_type {
            BoshTFLineType::Normal => Ok(LRComLineType::Normal),
            BoshTFLineType::Accelerate { amount } => {
                if amount == &1 {
                    Ok(LRComLineType::Accelerate)
                } else {
                    Err(anyhow!("lrcom cannot have acceleration amounts"))
                }
            }
            BoshTFLineType::Scenery => Ok(LRComLineType::Scenery),
        }
    }
}

impl From<&LRComLineType> for BoshTFLineType {
    fn from(line_type: &LRComLineType) -> BoshTFLineType {
        match line_type {
            LRComLineType::Normal => BoshTFLineType::Normal,
            LRComLineType::Accelerate => BoshTFLineType::Accelerate { amount: 1 },
            LRComLineType::Scenery => BoshTFLineType::Scenery,
        }
    }
}

impl TryFrom<&BoshTFLine> for LRComLine {
    type Error = Error;

    fn try_from(line: &BoshTFLine) -> Result<LRComLine> {
        Ok(LRComLine {
            id: 0,
            line_type: (&line.line_type)
                .try_into()
                .context("cannot convert line")?,
            x1: line.ends.0.location.0,
            y1: line.ends.0.location.1,
            x2: line.ends.1.location.0,
            y2: line.ends.1.location.1,
            flipped: line.flipped,
            left_extended: line.ends.0.extended,
            right_extended: line.ends.1.extended,
        })
    }
}

impl From<&LRComLine> for BoshTFLine {
    fn from(line: &LRComLine) -> BoshTFLine {
        Line::builder()
            .point(line.x1, line.y1)
            .extended(line.left_extended)
            .point(line.x2, line.y2)
            .extended(line.right_extended)
            .flipped(line.flipped)
            .line_type((&line.line_type).into())
            .build()
    }
}

impl TryFrom<&BoshTFEntity> for LRComEntity {
    type Error = Error;

    fn try_from(entity: &BoshTFEntity) -> Result<LRComEntity> {
        match entity {
            BoshTFEntity::Custom(_) => {
                Err(anyhow!("must be a boshsled to serialize to track.json"))
            }
            BoshTFEntity::BoshSled { position, velocity } => Ok(LRComEntity {
                start_position: position.into(),
                start_velocity: velocity.into(),
                remountable: 0,
            }),
        }
    }
}

impl From<&LRComEntity> for BoshTFEntity {
    fn from(entity: &LRComEntity) -> BoshTFEntity {
        BoshTFEntity::BoshSled {
            velocity: (&entity.start_velocity).into(),
            position: (&entity.start_position).into(),
        }
    }
}

impl From<&LRComTrack> for BoshTFTrack {
    fn from(track: &LRComTrack) -> BoshTFTrack {
        let entities = if let Some(riders) = &track.riders {
            riders
                .iter()
                .map(|ent| BoshTFEntity::BoshSled {
                    velocity: (&ent.start_velocity).into(),
                    position: (&ent.start_position).into(),
                })
                .collect()
        } else {
            vec![BoshTFEntity::BoshSled {
                position: (&track.start_position).into(),
                velocity: Vector2D(0.4, 0.0),
            }]
        };

        let lines = if let Some(lines) = &track.lines {
            lines.iter().map(|l| l.into()).collect()
        } else if let Some(lines) = &track.lines_array {
            lines
                .iter()
                .map(|l| {
                    eprintln!("{:?}", l);
                    let obj = &LRComLine::from(*l);
                    eprintln!("{:?}", obj);
                    let bosh = BoshTFLine::from(obj);
                    eprintln!("{:?}", obj);
                    bosh
                })
                .collect()
        } else {
            vec![]
        };

        BoshTFTrack {
            meta: Default::default(),
            entities,
            lines,
        }
    }
}

impl TryFrom<&BoshTFTrack> for LRComTrack {
    type Error = Error;

    fn try_from(track: &BoshTFTrack) -> Result<LRComTrack> {
        let first_entity = track
            .entities
            .get(0)
            .context("error converting boshtf into lr.com: no entites")?;
        let first_entity: LRComEntity = first_entity
            .try_into()
            .context("error converting boshtf into lr.com: bad entity")?;

        let mut riders: Vec<LRComEntity> = Vec::with_capacity(track.entities.len());
        for e in &track.entities {
            riders.push(
                e.try_into()
                    .context("error converting boshtf into lr.com: bad entity")?,
            );
        }

        let mut lines: Vec<LRComLine> = Vec::with_capacity(track.lines.len());
        for l in &track.lines {
            lines.push(l.try_into().context("error converting line")?);
        }

        Ok(LRComTrack {
            next_line_id: lines.len() as u64,
            label: None,
            creator: None,
            description: None,
            duration: None,
            version: "6.2".to_string(),
            audio: None,
            start_position: first_entity.start_position,
            riders: Some(riders),
            lines: Some(lines),
            lines_array: None,
        })
    }
}
