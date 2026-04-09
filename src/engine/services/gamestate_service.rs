use std::sync::{Arc, RwLock};

use crate::prelude::*;

/// Defines the gamestate service.
///
/// The gamestate service is used mainly for storing and reading of generic values usable by the
/// engine as a whole. All values are thread safe such that systems/services on seperate threads
/// may access them independently.
pub struct GamestateService {
    active_player: RwLock<u8>
}

impl GamestateService {
    /// Returns an [`Arc`] pointer to a new gamestate service using default values.
    pub fn new() -> Arc<Self> {
        Arc::new(
            Self {
                active_player: 0.into(),
            }
        )
    }

    /// Returns a clone of the stored value for the active player.
    pub fn get_active_player(&self) -> u8 {
        *self.active_player.read().unwrap_or_else(|_| {log!(Self, Critical, "Failed to readlock active_player..."); panic!()})
    }

    /// Sets the value of the stored active player.
    pub fn set_active_player(&self, active_player: u8) {
        *self.active_player.write().unwrap_or_else(|_| {log!(Self, Critical, "Failed to writelock active_player..."); panic!()}) = active_player;
    }
}
