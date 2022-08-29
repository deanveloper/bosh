//! Module for deserializing BoshTF, the Bosh Track Format

use bosh_rs::rider::{Entity, PointIndex};
use bosh_rs::{Line, LineType, Track, TrackMeta, Vector2D};
use serde::{Deserialize, Serialize};

pub type BoshTFLine = Line;
pub type BoshTFLineType = LineType;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BoshTFEntity {
    #[serde(rename = "boshSled")]
    BoshSled {
        #[serde(default = "default_starting_velocity")]
        velocity: Vector2D,
        #[serde(default = "default_starting_position")]
        position: Vector2D,
    },
    #[serde(rename = "custom")]
    Custom(Entity),
}

fn default_starting_position() -> Vector2D {
    Vector2D(0.4, 0.0)
}

fn default_starting_velocity() -> Vector2D {
    Vector2D(0.4, 0.0)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoshTFTrack {
    #[serde(default)]
    pub meta: TrackMeta,
    pub entities: Vec<BoshTFEntity>,
    pub lines: Vec<BoshTFLine>,
}

impl From<&BoshTFTrack> for Track {
    fn from(track: &BoshTFTrack) -> Track {
        Track::new(
            track.entities.iter().map(Entity::from).collect(),
            track.lines.clone(),
        )
    }
}

impl From<&BoshTFEntity> for Entity {
    fn from(entity: &BoshTFEntity) -> Self {
        match entity {
            BoshTFEntity::BoshSled { position, velocity } => {
                let mut bosh_sled = Entity::default_boshsled();
                bosh_sled.mutate_points(|point| {
                    point.location += *position;
                    point.previous_location = point.location - *velocity;
                });

                bosh_sled
            }
            BoshTFEntity::Custom(custom) => custom.clone(),
        }
    }
}

impl From<&Entity> for BoshTFEntity {
    fn from(entity: &Entity) -> Self {
        let peg = entity.points.get(&PointIndex::SledPeg);
        if matches!(peg, None) {
            return BoshTFEntity::Custom(entity.clone());
        }
        let peg = peg.unwrap();
        let peg_location = peg.location;
        let peg_velocity = peg.previous_location - peg.location;

        let default_boshsled: Entity = Entity::default_boshsled();
        let mut mapped_entity = entity.clone();
        for p in mapped_entity.points.values_mut() {
            let p_velocity = p.previous_location - p.location;
            if p_velocity != peg_velocity {
                return BoshTFEntity::Custom(entity.clone());
            }

            p.location -= peg_location;
            p.previous_location -= peg_location;
        }

        if default_boshsled != mapped_entity {
            BoshTFEntity::Custom(entity.clone())
        } else {
            BoshTFEntity::BoshSled {
                velocity: peg_velocity,
                position: peg_location,
            }
        }
    }
}
