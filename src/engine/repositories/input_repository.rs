use crate::prelude::*;
use core::f64;
use glam::bool;
use std::{collections::HashMap, sync::RwLock};

pub struct InputRepository {
    // keymaps: RwLock<HashMap<&'static str, bool>>,
    axismaps: RwLock<HashMap<&'static str, RwLock<f64>>>,
}

impl InputRepository {
    pub fn new() -> InputRepository {
        let mut keymaps: HashMap<&'static str, bool> = HashMap::new();
        keymaps.insert("PRIMARY", false);
        let mut axismaps: HashMap<&'static str, RwLock<f64>> = HashMap::new();
        axismaps.insert("CAMERAX", RwLock::new(0.0));
        axismaps.insert("CAMERAY", RwLock::new(0.0));
        axismaps.insert("FORWARD", RwLock::new(0.0));
        axismaps.insert("BACKWARD", RwLock::new(0.0));
        axismaps.insert("LEFT", RwLock::new(0.0));
        axismaps.insert("RIGHT", RwLock::new(0.0));

        InputRepository {
            // keymaps: RwLock::new(keymaps),
            axismaps: RwLock::new(axismaps),
        }
    }

    pub fn update_axis(&self, key: &'static str, value: f64) {
        if let Ok(axismaps) = self.axismaps.read() {
            if let Some(axis) = axismaps.get(key) {
                if let Ok(mut axis) = axis.write() {
                    *axis += value;

                    log!(
                        Self,
                        Low,
                        &format!("Updated axis: {}. New value: {}", key, axis).to_string()
                    );
                } else {
                    log!(Self, Critical, "Faild to writelock axis...");
                }
            } else {
                log!(Self, Critical, "Failed to get axis...");
            }
        } else {
            log!(Self, Critical, "Failed to readlock axismaps...");
        }
    }
}
