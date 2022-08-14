use std::collections::HashMap;

use anyhow::anyhow;
use bosh_rs::physics::entity_physics::PhysicsEntity;
use bosh_rs::rider::{Bosh, BoshSled, Entity, PointIndex, Sled};
use bosh_rs::Vector2D;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum EntityType {
    Bosh,
    Sled,
    BoshSled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableEntity {
    #[serde(alias = "entityType")]
    pub entity_type: EntityType,
    #[serde(default)]
    pub points: HashMap<String, (f64, f64)>,
}

impl SerializableEntity {
    pub fn new(entity: &Entity) -> SerializableEntity {
        let (etype, points) = match entity {
            Entity::Bosh(bosh) => {
                (EntityType::Bosh, bosh.points.clone())
            }
            Entity::Sled(sled) => {
                (EntityType::Sled, sled.points.clone())
            }
            Entity::BoshSled(bosh_sled) => {
                let mut points = bosh_sled.bosh.points.clone();
                points.extend(bosh_sled.sled.points.clone());

                (EntityType::BoshSled, points)
            }
        };

        let points_serialized: HashMap<String, (f64, f64)> = points.iter().map(|(idx, point)| {
            let loc = point.location;

            (point_index_to_string(idx), (loc.0, loc.1))
        }).collect();

        SerializableEntity {
            entity_type: etype,
            points: points_serialized,
        }
    }
}

impl<E: From<anyhow::Error>> Into<Result<Entity, E>> for SerializableEntity {
    fn into(self) -> Result<Entity, E> {
        match self.entity_type {
            EntityType::Bosh => {
                let mut bosh: Bosh = Default::default();
                for point in self.points {
                    bosh.point_at_mut(string_to_point_index(&point.0)?).location = Vector2D(point.1.0, point.1.1);
                }
                Ok(Entity::Bosh(bosh))
            }
            EntityType::Sled => {
                let mut sled: Sled = Default::default();
                for point in self.points {
                    sled.point_at_mut(string_to_point_index(&point.0)?).location = Vector2D(point.1.0, point.1.1);
                }
                Ok(Entity::Sled(sled))
            }
            EntityType::BoshSled => {
                let mut bosh_sled: BoshSled = Default::default();

                for point in self.points {
                    bosh_sled.point_at_mut(string_to_point_index(&point.0)?).location = Vector2D(point.1.0, point.1.1);
                }
                Ok(Entity::BoshSled(bosh_sled))
            }
        }
    }
}

fn string_to_point_index(s: &str) -> Result<PointIndex, anyhow::Error> {
    match s {
        "BoshLeftFoot" => Ok(PointIndex::BoshLeftFoot),
        "BoshRightFoot" => Ok(PointIndex::BoshRightFoot),
        "BoshLeftHand" => Ok(PointIndex::BoshLeftHand),
        "BoshRightHand" => Ok(PointIndex::BoshRightHand),
        "BoshShoulder" => Ok(PointIndex::BoshShoulder),
        "BoshButt" => Ok(PointIndex::BoshButt),
        "SledPeg" => Ok(PointIndex::SledPeg),
        "SledTail" => Ok(PointIndex::SledTail),
        "SledNose" => Ok(PointIndex::SledNose),
        "SledRope" => Ok(PointIndex::SledRope),
        _ => Err(anyhow!("")),
    }
}

fn point_index_to_string(idx: &PointIndex) -> String {
    match idx {
        PointIndex::BoshLeftFoot => "BoshLeftFoot",
        PointIndex::BoshRightFoot => "BoshRightFoot",
        PointIndex::BoshLeftHand => "BoshLeftHand",
        PointIndex::BoshRightHand => "BoshRightHand",
        PointIndex::BoshShoulder => "BoshShoulder",
        PointIndex::BoshButt => "BoshButt",
        PointIndex::SledPeg => "SledPeg",
        PointIndex::SledTail => "SledTail",
        PointIndex::SledNose => "SledNose",
        PointIndex::SledRope => "SledRope",
    }.to_owned()
}