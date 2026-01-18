use crate::prelude::*;
use std::{collections::HashMap, sync::RwLock};

use crate::engine::services::entity_service::entities::entity::Entity;
pub struct EntityRepository {
    entities: RwLock<HashMap<usize, Box<dyn Entity>>>,
    last_id: RwLock<usize>,
}

impl EntityRepository {
    pub fn new() -> EntityRepository {
        EntityRepository {
            entities: RwLock::new(HashMap::new()),
            last_id: RwLock::new(0),
        }
    }

    pub fn add_entity(&self, entity: Box<dyn Entity>) -> usize {
        if let Ok(entity_id) = self.last_id.write() {
            let entity_id = *entity_id + 1;

            if let Ok(mut entities) = self.entities.write() {
                entities.insert(entity_id, entity);
                entity_id
            } else {
                log!(Self, Critical, "Failed to writelock entities...");
                panic!()
            }
        } else {
            log!(Self, Critical, "Failed to writelock last_id...");
            panic!()
        }
    }

    /*
    pub fn remove_entity(&mut self, entity_id: &usize) {
        //[TO-DO]: Add some error checking or logging
        self.entities.remove(entity_id);
    }
    */

    /*
    pub fn get_entity(&mut self, entity_id: &usize) -> &mut Box<dyn Entity> {
        //[TO-DO]: Add some error checking or logging
        self.entities.get_mut(entity_id).unwrap()
    }
    */
}
