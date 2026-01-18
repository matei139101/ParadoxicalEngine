use crate::{engine::utils::structs::transform::Transform, prelude::*};
use std::{collections::HashMap, sync::RwLock};

pub struct EntityRepository {
    entities: RwLock<HashMap<usize, RwLock<String>>>,
    transforms: RwLock<HashMap<usize, RwLock<Transform>>>,
    last_id: RwLock<usize>,
}

impl EntityRepository {
    pub fn new() -> EntityRepository {
        EntityRepository {
            entities: Default::default(),
            transforms: Default::default(),
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

    pub fn add_entity(&self, id:usize, name: String) {
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
}
