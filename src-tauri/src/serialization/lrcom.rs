//! Module for deserializing linerider.com tracks, aka ".track.json"

use std::borrow::Borrow;

use anyhow::{anyhow, Context, Error};
use bosh_rs::{Line, Vector2D};
use serde::{Deserialize, Serialize};

use crate::serialization::boshtf::{BoshTFEntity, BoshTFLine, BoshTFLineType, BoshTFTrack};

type Result<T> = anyhow::Result<T>;

#[derive(Clone, Serialize, Deserialize)]
pub struct LRComTrack {
    next_line_id: u64,
    label: String,
    creator: String,
    description: String,
    duration: u64,
    version: String,
    audio: Option<()>,
    #[serde(rename = "startPosition")]
    start_position: LRComVec2,
    riders: Vec<LRComEntity>,
    lines: Vec<LRComLine>,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct LRComEntity {
    #[serde(rename = "startPosition")]
    start_position: LRComVec2,
    #[serde(rename = "startVelocity")]
    start_velocity: LRComVec2,
    remountable: bool,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct LRComVec2 {
    x: f64,
    y: f64,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, Copy, Clone)]
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
                remountable: false,
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
    fn from(track: &LRComTrack) -> Self {
        let entities = if track.riders.len() > 0 {
            track
                .riders
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

        let lines = track.lines.iter().map(|l| l.into()).collect();

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
            label: "created with bosh".to_string(),
            creator: "created with bosh".to_string(),
            description: "created with bosh".to_string(),
            duration: 0,
            version: "6.2".to_string(),
            audio: None,
            start_position: first_entity.start_position,
            riders,
            lines,
        })
    }
}
