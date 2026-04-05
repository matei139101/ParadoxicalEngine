use crate::prelude::*;

/**
 * A repository for all engine inputs.
 *
 * Containes the values of all axes mapped and watched by the engine in a thread safe manner. Only
 * provides access to these values without any additional functionality.
 */
pub struct InputRepository {
    // keymaps: RwLock<HashMap<&'static str, bool>>,
    axismaps: RwLock<HashMap<&'static str, RwLock<f64>>>,
}

impl InputRepository {
    /**
     * Returns a new input repository using default values.
     *
     * # Example
     * ```
     * let repository = InputRepository::new();
     * ```
     */
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

    /**
     * Returns the cloned value of a specific axis filtered by key.
     *
     * Returns `Some(f64)` or `None()` depending on if the keymap exists.
     *
     * # Example
     * ```
     * let axis_value = repository.get_axis("CAMERA_Y");
     *
     * match axis_value {
     *  Some(y) => //Use axis value,
     *  None() => //Handle missing axis,
     * }
     * ```
     */
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

    /**
     * Updates a specific axis filtered by name with the given value. This adds the existing axis
     * value with the provided one. To entirely overwrite the value see [`InputRepository::set_axis`].
     *
     * # Example
     * ```
     * let axis_value = repository.get_axis("CAMERA_Y").unwrap() + 10;
     *
     * repository.update_axis("CAMERA_Y") + 10;
     * ```
     */
    pub fn update_axis(&self, key: &'static str, value: f64) {
        if let Ok(axismaps) = self.axismaps.read() {
            if let Some(axis) = axismaps.get(key) {
                if let Ok(mut axis) = axis.write() {
                    *axis += value;
                    /*
                    log!(
                        Self,
                        Low,
                        &format!("Updated axis: {}. New value: {}", key, axis).to_string()
                    );
                    */
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

    /**
     * Sets the values of a specific axis with the given key. This entirely overwrites the existing
     * value with the given one. To add values to existing axes, see [`InputRepository::update_axis`].
     *
     * # Example
     * ```
     * let axis_value = 128.48;
     *
     * repository.set_axis("CAMERA_Y", axis_value);
     * ```
     */
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
