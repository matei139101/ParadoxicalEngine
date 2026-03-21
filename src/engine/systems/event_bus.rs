use crate::prelude::*;

type Callback = Box<dyn Fn(&dyn Any) + Send + Sync>;

pub struct EventBus {
    observers: RwLock<HashMap<TypeId, Vec<Callback>>>,
}

impl EventBus {
    pub fn new() -> Arc<EventBus> {
        Arc::new(EventBus {
            observers: RwLock::new(HashMap::new()),
        })
    }

    pub fn observe<E: 'static + Send + Sync>(&self, callback: Callback) {
        let mut observers = self.observers.write().unwrap();

        observers
            .entry(TypeId::of::<E>())
            .or_default()
            .push(callback);
    }

    pub fn emit(&self, event: Box<dyn Any + Send + Sync>) {
        let observers = self.observers.read().unwrap();

        if let Some(callbacks) = observers.get(&(*event).type_id()) {
            for callback in callbacks {
                callback(event.as_ref());
            }
        }
    }

    pub fn run(
        self_ptr: Arc<EventBus>,
        async_receiver: Receiver<Box<dyn Any + Send + Sync>>,
    ) {
        while let Ok(event) = async_receiver.recv() {
            let bus = self_ptr.clone();
            thread::spawn(move || {
                bus.emit(event);
            });
        }
    }
}
