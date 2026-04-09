use crate::prelude::*;

type Callback = Box<dyn Fn(&dyn Any) + Send + Sync>;

/// Defines the event bus system.
///
/// The event bus is a system usable by most if not all components of the engine to handle events.
/// A component may assign its own handler to a specific event which will be called any time
/// another component emits such an event.
///
/// A rework is currently being considered to better make use of multithreading and easier to use
/// events and handlers.
pub struct EventBus {
    observers: RwLock<HashMap<TypeId, Vec<Callback>>>,
}

impl EventBus {
    /// Returns a new event bus created using default values.
    pub fn new() -> Self {
        EventBus {
            observers: RwLock::new(HashMap::new()),
        }
    }

    /// Assigns a handler to a type of event. Usable by engine components to add custom
    /// functionality to an event.
    pub fn observe<E: 'static + Send + Sync>(&self, callback: Callback) {
        let mut observers = self.observers.write().unwrap();

        observers
            .entry(TypeId::of::<E>())
            .or_default()
            .push(callback);
    }

    /// Calls the handlers for each event emitted and processes them.
    fn emit(&self, event: Box<dyn Any + Send + Sync>) {
        let observers = self.observers.read().unwrap();

        if let Some(callbacks) = observers.get(&(*event).type_id()) {
            for callback in callbacks {
                callback(event.as_ref());
            }
        }
    }

    /// Runs the event bus. Emitting any events sent on the event bus channel.
    pub fn run(self: Arc<Self>, receiver: Receiver<Box<dyn Any + Send + Sync>>) {
        while let Ok(event) = receiver.recv() {
            self.emit(event);
        }
    }
}
