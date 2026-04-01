use winit::{event::{ElementState, KeyEvent}, keyboard::{KeyCode, PhysicalKey}};

use crate::prelude::*;

pub struct InputService {
    repositories: Arc<Repositories>,
}

impl InputService {
    pub fn new(
        repositories: Arc<Repositories>,
    ) -> Arc<InputService> {
        let input_service = InputService { repositories };

        Arc::new(input_service)
    }

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

    pub fn input_axis(&self, event: DeviceEvent) {
        let input_repo = self.repositories.get_input_repository();

        if let DeviceEvent::MouseMotion { delta } = event {
            input_repo.update_axis("CAMERAY", delta.0);
            input_repo.update_axis("CAMERAX", delta.1);
        }

    }
}
