use std::thread::{sleep};
use crate::prelude::{*};

pub struct Scheduler {
    sender: Sender<Box<dyn Any + Send + Sync>>,
    services: Arc<Services>,
    event_bus: Arc<EventBus>,
}

impl Scheduler {
    pub fn new(sender: Sender<Box<dyn Any + Send + Sync>>, services: Arc<Services>, event_bus: Arc<EventBus>) -> Scheduler {
        Scheduler { sender, services, event_bus }
    }

    pub fn start(&self, async_receiver: Receiver<Box<dyn Any + Send + Sync>>) {
        self.make_event_bus_thread(async_receiver);
        self.make_auxilliary_thread();
        self.make_service_thread();
        self.make_window_thread();
        self.make_window_thread();
    }

    fn make_service_thread(&self) {
        log!(Self, Critical, "Starting service thread.");
        let services = Arc::clone(&self.services);

        let _ = thread::Builder::new().name("Synchronizer".to_string()).spawn(move || {
            while !services.get_vulkan_service().is_ready() {
                sleep(Duration::from_millis(10));
            }

            loop {
                services.get_vulkan_service().update();
                services.get_entity_service().update();
            }
        });
    }

    fn make_auxilliary_thread(&self) {
        log!(Self, Critical, "Starting auxilliary thread.");
        let _ = thread::Builder::new().name("Auxilliary".to_string()).spawn(move || {
            loop {
                LOGGER.update();
                DASHBOARD.update();
            }
        });
    }

    fn make_event_bus_thread(&self, receiver: Receiver<Box<dyn Any + Send + Sync>>) {
        log!(Self, Dev, "Starting eventbus thread.");
        let event_bus = Arc::clone(&self.event_bus);

        let _ = thread::Builder::new().name("EventBus".to_string()).spawn(move || {
            event_bus.run(receiver);
        });
    }

    fn make_window_thread(&self) {
        log!(Self, Dev, "Starting window thread.");

        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app = App::new(self.services.clone(), self.sender.clone());

        let _ = event_loop.run_app(&mut app);
    }
}
