use crate::prelude::*;
use std::panic;

/**
 * A repository for all entities to be used by the engine.
 *
 * Contains the different components each entity has, and the values of said entity components, in
 * a thread safe manner. Only provides access to these values without any additional functionality.
 */
pub struct EntityRepository {
    entities: RwLock<HashMap<usize, String>>,
    transforms: RwLock<HashMap<usize, Transform>>,
    controllers: RwLock<HashMap<usize, i16>>,

    update_functions: RwLock<HashMap<usize, fn(Arc<Repositories>)>>,
    last_id: RwLock<usize>,
}

impl EntityRepository {
    /**
     * Returns a new entity repository using default values.
     *
     * # Example
     * ```
     * let repository = EntityRepository::new();
     * ```
     */
    pub fn new() -> EntityRepository {
        EntityRepository {
            entities: Default::default(),
            transforms: Default::default(),
            controllers: Default::default(),
            
            update_functions: Default::default(),
            last_id: RwLock::new(0),
        }
    }

    /**
     * Obtains a fresh, unused ID for creating and tracking of new entities.
     *
     * # Example
     * ```
     * let id = repository.get_id();
     * ```
     */
    pub fn get_id(&self) -> Option<usize> {
        if let Ok(mut id) = self.last_id.write() {
            *id += 1;
            Some(*id)
        } else {
            log!(Self, Critical, "Failed to writelock last_id...");
            None
        }
    }

    /**
     * Adds an entity to the repository with the given ID and name.
     *
     * # Example
     * ```
     * let id = repository.get_id();
     * repository.add_entity(id, "Hot Potato");
     * ```
     */
    pub fn add_entity(&self, id: usize, name: String) {
        if let Ok(mut entities) = self.entities.write() {
            entities.insert(id, name);
            log!(Self, Medium, "Added an entity.");
        } else {
            log!(Self, Critical, "Failed to writelock entities...");
        }
    }

    /**
     * Sets the transform of a the specific entity with the given ID.
     *
     * # Example
     * ```
     * let id = repository.get_id();
     * let transform = Transform::default();
     *
     * repository.set_transform(id, transform);
     * ```
     */
    pub fn set_transform(&self, id: usize, transform: Transform) {
        if let Ok(mut transforms) = self.transforms.write() {
            transforms.insert(id, transform);
            log!(Self, Medium, "Added a transform.");
        } else {
            log!(Self, Critical, "Failed to writelock transforms...");
        }
    }

    /**
     * Returns a cloned transform of a specific entity filtered by ID.
     *
     * The return is `Some(Transform)` or `None` depending on if a transform for the given entity
     * exists.
     *
     * # Example
     * ```
     * let transform = repository.get_transform(1);
     *
     * match transform {
     *  Some(t) => //Use the transform,
     *  None() => // Handle missing entity,
     * }
     * ```
     */
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

    /**
     * Adds a controller for a specific entity with the given ID and for the given player number.
     *
     * # Example
     * ```
     * let id = repository.get_id();
     * let player_number = 2;
     *
     * repository.add_controller(id, player_number);
     * ```
     */
    pub fn add_controller(&self, id: usize, player_number: i16) {
        if let Ok(mut controllers) = self.controllers.write() {
            controllers.insert(id, player_number);
            log!(Self, Medium, "Added a player.");
        } else {
            log!(Self, Critical, "Failed to writelock controllers...");
        }
    }

    /**
     * Adds a function to the repository for a specific entity to be used during the
     * [`EntityService`] update cycle.
     *
     * Temporary placeholder! Subject to change if not complete removal in the near future.
     */
    pub fn add_update_function(&self, id: usize, function: fn(Arc<Repositories>)) {
        if let Ok(mut update_functions) = self.update_functions.write() {
            update_functions.insert(id, function);
            log!(Self, Medium, "Added a transform.");
        } else {
            log!(Self, Critical, "Failed to writelock transforms...");
        }
    }

    /**
     * Gets all functions stored to be used during the
     * [`EntityService`] update cycle.
     *
     * Temporary placeholder! Subject to change if not complete removal in the near future.
     */
    pub fn get_update_functions(&self) -> Option<HashMap<usize, fn(Arc<Repositories>)>> {
        if let Ok(update_functions) = self.update_functions.read() {
            Some(update_functions.clone())
        } else {
            None
        }
    }

    /**
     * Returns a cloned transform of the camera component linked to a specific entity filterable by
     * ID.
     *
     * The return is `Some(Transform)` or `None()` depending on if a cameera for the given entity
     * exists.
     *
     * # Example
     * ```
     * let transform = repository.get_camera_transform();
     *
     * match transform {
     *  Some(t) => //Use the transform,
     *  None() => //Handle missing entity,
     * }
     * ```
     */
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

    /**
     * Returns the ID of the entity linked to a given player number.
     *
     * The return is `Some(usize)` or `None()` depending on if a controller for the given entity
     * exists.
     *
     * # Example
     * ```
     * let entity_id = repository.get_player_controller(2);
     *
     * match entity_id {
     *  Some(id) => //Use the id,
     *  None() => //Handle missing entity.
     * }
     * ```
     */
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
