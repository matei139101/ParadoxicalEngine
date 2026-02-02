use crate::prelude::*;

pub struct InputService {
    repositories: Arc<Repositories>,
}

impl InputService {
    pub fn new(
        repositories: Arc<Repositories>,
        input_reciever: UnboundedReceiver<DeviceEvent>,
    ) -> Arc<Mutex<InputService>> {
        let input_service = InputService { repositories };
        input_service.run(input_reciever);

        Arc::new(Mutex::new(input_service))
    }

    fn run(&self, input_reciever: UnboundedReceiver<DeviceEvent>) {
        let mut stream = UnboundedReceiverStream::new(input_reciever);
        let input_repo = self.repositories.get_input_repository();

        thread::spawn(move || {
            let async_runtime = tokio::runtime::Runtime::new().unwrap();
            async_runtime.block_on(async {
                while let Some(input) = stream.next().await {
                    if let DeviceEvent::MouseMotion { delta } = input {
                        input_repo.update_axis("CAMERAY", delta.1);
                        input_repo.update_axis("CAMERAX", delta.0);
                    }
                }
            });
        });
    }
}
