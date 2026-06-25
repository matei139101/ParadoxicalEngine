use winit::{event::{ElementState, KeyEvent}, keyboard::{KeyCode, PhysicalKey}};

use crate::prelude::*;

/// A service responsible for handling all input events from the OS such as translating raw mouse and keyboard input into axis values.
///
/// Makes use of an [`InputRepository`] to store axis values.
pub struct InputService {
    repositories: Arc<Repositories>,
}

impl InputService {
     /// Returns an [`Arc`] pointer to a new input service which uses the given repositories for
     /// storing and reading data.
    pub fn new(
        repositories: Arc<Repositories>,
    ) -> Arc<InputService> {
        let input_service = InputService { repositories };

        Arc::new(input_service)
    }
     
    /// Translates raw key events into axis values then stores them on the input repository for use.
    pub fn input_key(&self, event: KeyEvent) {
        let input_repo = self.repositories.get_input_repository();
        if event.state == ElementState::Pressed {
            match event.physical_key {
                PhysicalKey::Code(KeyCode::KeyW) => { input_repo.set_axis("FORWARD", 1.0); }
                PhysicalKey::Code(KeyCode::KeyA) => { input_repo.set_axis("RIGHT", -1.0); }
                PhysicalKey::Code(KeyCode::KeyS) => { input_repo.set_axis("FORWARD", -1.0); }
                PhysicalKey::Code(KeyCode::KeyD) => { input_repo.set_axis("RIGHT", 1.0); }
                _ => {}
            }
        } else {
            match event.physical_key {
                PhysicalKey::Code(KeyCode::KeyW) => { input_repo.set_axis("FORWARD", 0.0); }
                PhysicalKey::Code(KeyCode::KeyA) => { input_repo.set_axis("RIGHT", 0.0); }
                PhysicalKey::Code(KeyCode::KeyS) => { input_repo.set_axis("FORWARD", 0.0); }
                PhysicalKey::Code(KeyCode::KeyD) => { input_repo.set_axis("RIGHT", 0.0); }
                _ => {}
            }
        }
    }

    /// Translates raw device events from mouse input into axis values then stores them on the input
    /// repository for later use.
    pub fn input_axis(&self, event: DeviceEvent) {
        let input_repo = self.repositories.get_input_repository();

        if let DeviceEvent::MouseMotion { delta } = event {
            input_repo.update_axis("CAMERAY", delta.0);
            input_repo.update_axis("CAMERAX", delta.1);
        }

    }
}
