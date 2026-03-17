use crate::prelude::*;
use std::panic;

pub struct EntityRepository {
    entities: RwLock<HashMap<usize, String>>,
    transforms: RwLock<HashMap<usize, Transform>>,
    controllers: RwLock<HashMap<usize, i16>>,

    update_functions: RwLock<HashMap<usize, fn(Arc<Repositories>)>>,
    last_id: RwLock<usize>,
}

impl EntityRepository {
    pub fn new() -> EntityRepository {
        EntityRepository {
            entities: Default::default(),
            transforms: Default::default(),
            controllers: Default::default(),
            
            update_functions: Default::default(),
            last_id: RwLock::new(0),
        }
    }

    pub fn get_id(&self) -> Option<usize> {
        if let Ok(mut id) = self.last_id.write() {
            *id += 1;
            Some(*id)
        } else {
            log!(Self, Critical, "Failed to writelock last_id...");
            None
        }
    }

    pub fn add_entity(&self, id: usize, name: String) {
        if let Ok(mut entities) = self.entities.write() {
            entities.insert(id, name);
            log!(Self, Medium, "Added an entity.");
        } else {
            log!(Self, Critical, "Failed to writelock entities...");
        }
    }

    pub fn set_transform(&self, id: usize, transform: Transform) {
        if let Ok(mut transforms) = self.transforms.write() {
            transforms.insert(id, transform);
            log!(Self, Medium, "Added a transform.");
        } else {
            log!(Self, Critical, "Failed to writelock transforms...");
        }
    }

    pub fn get_transform(&self, id: usize) -> Option<Transform> {
        if let Ok(transforms) = self.transforms.read() {
            if let Some(transform) = transforms.get(&id) {
                Some(transform.clone())
            } else {
                log!(Self, Critical, "Failed to find transform by id...");
                None
            }
        } else {
            log!(Self, Critical, "Failed to readlock transforms...");
            None
        }
    }

    pub fn add_controller(&self, id: usize, player_number: i16) {
        if let Ok(mut controllers) = self.controllers.write() {
            controllers.insert(id, player_number);
            log!(Self, Medium, "Added a player.");
        } else {
            log!(Self, Critical, "Failed to writelock controllers...");
        }
    }

    pub fn add_update_function(&self, id: usize, function: fn(Arc<Repositories>)) {
        if let Ok(mut update_functions) = self.update_functions.write() {
            update_functions.insert(id, function);
            log!(Self, Medium, "Added a transform.");
        } else {
            log!(Self, Critical, "Failed to writelock transforms...");
        }
    }

    pub fn get_update_functions(&self) -> Option<HashMap<usize, fn(Arc<Repositories>)>> {
        if let Ok(update_functions) = self.update_functions.read() {
            Some(update_functions.clone())
        } else {
            None
        }
    }

    pub fn get_camera_transform(&self, player_id: i16) -> Option<Transform> {
        if let Ok(controllers) = self.controllers.read() {
            for (entity_id, controller_id) in controllers.iter() {
                if *controller_id == player_id {
                    if let Ok(transforms) = self.transforms.read() {
                        if let Some(transform) = transforms.get(entity_id) {
                            return Some(transform.clone());
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
                Medium,
                &format!("No entity found with player id: {}", player_id).to_string()
            );
            Some(Transform {
                position: Vec3::new(0.0, 0.0, -5.0),
                rotation: Vec3::new(0.0, 0.0, 0.0),
            })
        } else {
            log!(Self, Critical, "Failed to readlock controllers...");
            None
        }
    }

    pub fn get_player_controller(&self, player_id: i16) -> Option<usize> {
        if let Ok(controllers) = self.controllers.read() {
            for (entity_id, controller_id) in controllers.iter() {
                if controller_id == &player_id {
                    return Some(*entity_id);
                }
            }

            log!(Self, Critical, "Failed to get entityId of controller...");
            None
        } else {
            log!(Self, Critical, "Failed to readlock controllers...");
            None
        }
    }
}
