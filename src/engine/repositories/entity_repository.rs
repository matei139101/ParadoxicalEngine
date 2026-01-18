use glam::vec3;

use crate::{
    engine::utils::structs::transform::{self, Transform},
    prelude::*,
};
use core::panic;
use std::{collections::HashMap, sync::RwLock};

pub struct EntityRepository {
    entities: RwLock<HashMap<usize, RwLock<String>>>,
    transforms: RwLock<HashMap<usize, RwLock<Transform>>>,
    controllers: RwLock<HashMap<usize, i16>>,
    last_id: RwLock<usize>,
}

impl EntityRepository {
    pub fn new() -> EntityRepository {
        EntityRepository {
            entities: Default::default(),
            transforms: Default::default(),
            controllers: Default::default(),
            last_id: RwLock::new(0),
        }
    }

    pub fn get_id(&self) -> usize {
        if let Ok(mut id) = self.last_id.write() {
            *id += 1;
            *id
        } else {
            log!(Self, Critical, "Failed to writelock last_id...");
            panic!();
        }
    }

    pub fn add_entity(&self, id: usize, name: String) {
        if let Ok(mut entities) = self.entities.write() {
            entities.insert(id, RwLock::new(name));
            log!(Self, Medium, "Added an entity.");
        } else {
            log!(Self, Critical, "Failed to writelock entities...");
            panic!()
        }
    }

    pub fn add_transform(&self, id: usize, transform: Transform) {
        if let Ok(mut transforms) = self.transforms.write() {
            transforms.insert(id, RwLock::new(transform));
            log!(Self, Medium, "Added a transform.");
        } else {
            log!(Self, Critical, "Failed to writelock transforms...");
            panic!();
        }
    }

    pub fn add_controller(&self, id: usize, player_number: i16) {
        if let Ok(mut controllers) = self.controllers.write() {
            controllers.insert(id, player_number);
            log!(Self, Medium, "Added a player.");
        } else {
            log!(Self, Critical, "Failed to writelock controllers...");
            panic!();
        }
    }

    pub fn get_camera_transform(&self, player_id: i16) -> Transform {
        if let Ok(controllers) = self.controllers.read() {
            for (entity_id, controller_id) in controllers.iter() {
                if *controller_id == player_id {
                    if let Ok(transforms) = self.transforms.read() {
                        if let Some(transform) = transforms.get(entity_id) {
                            if let Ok(transform) = transform.read() {
                                return transform.clone();
                            }
                        } else {
                            log!(
                                Self,
                                Critical,
                                &format!("Failed to get entity transform with id: {}", player_id)
                                    .to_string()
                            );
                        }
                    } else {
                        log!(Self, Critical, "Failed to readlock transforms...");
                        panic!();
                    }
                }
            }
            log!(
                Self,
                Critical,
                &format!("No entity found with player id: {}", player_id).to_string()
            );
            Transform {
                position: vec3(0.0, 0.0, -5.0),
                rotation: vec3(0.0, 0.0, 0.0),
            }
        } else {
            log!(Self, Critical, "Failed to readlock controllers...");
            panic!();
        }
    }
}
