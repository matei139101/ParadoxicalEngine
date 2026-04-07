use std::sync::{Arc, RwLock};

use crate::prelude::*;

/**
 * A service responsible for all engine-wide values. Currently unfinished and under active
 * development.
 */
pub struct GamestateService {
    active_player: RwLock<u8>
}

impl GamestateService {
    pub fn new() -> Arc<Self> {
        Arc::new(
            Self {
                active_player: 0.into(),
            }
        )
    }

    pub fn get_active_player(&self) -> u8 {
        *self.active_player.read().unwrap_or_else(|_| {log!(Self, Critical, "Failed to readlock active_player..."); panic!()})
    }

    pub fn set_active_player(&self, active_player: u8) {
        *self.active_player.write().unwrap_or_else(|_| {log!(Self, Critical, "Failed to writelock active_player..."); panic!()}) = active_player;
    }
}
