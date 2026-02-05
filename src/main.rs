use crate::prelude::*;
mod engine;
mod prelude;
mod resources;

fn main() {
    let (async_sender, async_receiver) = mpsc::unbounded_channel::<Box<dyn Any + Send + Sync>>();
    let (input_sender, input_receiver) = mpsc::unbounded_channel::<DeviceEvent>();

    let event_bus = EventBus::new();

    let repositories: Arc<Repositories> = Arc::new(Repositories::new());
    let services: Arc<Services> = Arc::new(Services::new(repositories, event_bus.clone(), async_sender.clone(), input_receiver));
    let app = make_app(async_sender.clone(), input_sender.clone());


    let synchronizer = Synchronizer::new(services);
    let synchronizer_handle = synchronizer.start();
    start_event_bus_thread(event_bus, async_receiver);
    start_window_thread(app);
}

fn make_app(
    async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
    input_sender: UnboundedSender<DeviceEvent>,
) -> (App, EventLoop<()>) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let app = App::new(async_sender, input_sender);
    (app, event_loop)
}

fn start_event_bus_thread(
    event_bus_ptr: Arc<EventBus>,
    async_receiver: UnboundedReceiver<Box<dyn Any + Send + Sync>>,
) {
    thread::spawn(move || {
        log!(High, "Starting async runtime");
        let async_runtime = tokio::runtime::Runtime::new().unwrap();

        async_runtime.block_on(async {
            EventBus::run(event_bus_ptr.clone(), async_receiver).await;
        })
    });
}

fn start_window_thread(mut app: (App, EventLoop<()>)) {
    let _ = app.1.run_app(&mut app.0);
}
