use winit::{event::{ElementState, KeyEvent}, keyboard::{KeyCode, PhysicalKey}};

use crate::prelude::*;

pub struct InputService {
    repositories: Arc<Repositories>,
}

impl InputService {
    pub fn new(
        repositories: Arc<Repositories>,
        input_receiver: UnboundedReceiver<(Option<KeyEvent>, Option<DeviceEvent>)>,
    ) -> Arc<Mutex<InputService>> {
        let input_service = InputService { repositories };
        input_service.run(input_receiver);

        Arc::new(Mutex::new(input_service))
    }

    fn run(&self, input_receiver: UnboundedReceiver<(Option<KeyEvent>, Option<DeviceEvent>)>) {
        let mut stream = UnboundedReceiverStream::new(input_receiver);
        let input_repo = self.repositories.get_input_repository();

        thread::spawn(move || {
            let async_runtime = tokio::runtime::Runtime::new().unwrap();
            async_runtime.block_on(async {
                while let Some(input) = stream.next().await {
                    if let Some(DeviceEvent::MouseMotion { delta }) = input.1 {
                        input_repo.update_axis("CAMERAY", delta.0);
                        input_repo.update_axis("CAMERAX", delta.1);
                    }

                    if let Some(key) = input.0 {
                        if key.state == ElementState::Pressed {
                            match key.physical_key {
                                PhysicalKey::Code(KeyCode::KeyW) => { input_repo.set_axis("FORWARD", 1.0); }
                                PhysicalKey::Code(KeyCode::KeyA) => { input_repo.set_axis("RIGHT", -1.0); }
                                PhysicalKey::Code(KeyCode::KeyS) => { input_repo.set_axis("FORWARD", -1.0); }
                                PhysicalKey::Code(KeyCode::KeyD) => { input_repo.set_axis("RIGHT", 1.0); }
                                _ => {}
                            }
                        } else {
                            match key.physical_key {
                                PhysicalKey::Code(KeyCode::KeyW) => { input_repo.set_axis("FORWARD", 0.0); }
                                PhysicalKey::Code(KeyCode::KeyA) => { input_repo.set_axis("RIGHT", 0.0); }
                                PhysicalKey::Code(KeyCode::KeyS) => { input_repo.set_axis("FORWARD", 0.0); }
                                PhysicalKey::Code(KeyCode::KeyD) => { input_repo.set_axis("RIGHT", 0.0); }
                                _ => {}
                            }
                        }
                    }
                }
            });
        });
    }
}
