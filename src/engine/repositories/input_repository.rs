use crate::prelude::*;

/// Defines the input repository.
///
/// The input repository stores the values for all axes and keymaps to be used by different
/// systems/services thread safely.
pub struct InputRepository {
    // keymaps: RwLock<HashMap<&'static str, bool>>,
    axismaps: RwLock<HashMap<&'static str, RwLock<f64>>>,
}

impl InputRepository {
    /// Returns a new input repository created with default values.
    pub fn new() -> InputRepository {
        let mut keymaps: HashMap<&'static str, bool> = HashMap::new();
        keymaps.insert("PRIMARY", false);
        let mut axismaps: HashMap<&'static str, RwLock<f64>> = HashMap::new();
        axismaps.insert("CAMERAX", RwLock::new(0.0));
        axismaps.insert("CAMERAY", RwLock::new(0.0));
        axismaps.insert("FORWARD", RwLock::new(0.0));
        axismaps.insert("RIGHT", RwLock::new(0.0));

        InputRepository {
            // keymaps: RwLock::new(keymaps),
            axismaps: RwLock::new(axismaps),
        }
    }

    /// Returns either [`Some`] containing the current value of the axis or [`None`] if the indexed
    /// axis doesn't exist.
    pub fn get_axis(&self, key: &'static str) -> Option<f64> {
        if let Ok(axismaps) = self.axismaps.read() {
            if let Some(axis) = axismaps.get(key) {
                if let Ok(axis) = axis.read() {
                    Some(*axis)
                } else {
                    log!(Self, Critical, "Faild to readlock axis...");
                    None
                }
            } else {
                log!(Self, Critical, "Failed to get axis...");
                None
            }
        } else {
            log!(Self, Critical, "Failed to readlock axismaps...");
            None
        }
    }

    /// Adds the given value to the existing axis value if the indexed axis exists.
    pub fn update_axis(&self, key: &'static str, value: f64) {
        if let Ok(axismaps) = self.axismaps.read() {
            if let Some(axis) = axismaps.get(key) {
                if let Ok(mut axis) = axis.write() {
                    *axis += value;
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

    /// Updates the axis value by overwriting it with the given value if the indexed axis exists.
    pub fn set_axis(&self, key: &'static str, value: f64) {
        if let Ok(axismaps) = self.axismaps.read() {
            if let Some(axis) = axismaps.get(key) {
                if let Ok(mut axis) = axis.write() {
                    *axis = value;
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
