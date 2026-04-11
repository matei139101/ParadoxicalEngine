use crate::prelude::*;

/// Defines the trait for all engine services.
///
/// Services are long lived components of the game engine which are meant to be largely
/// self-sufficient aside from reading or writing to repositories when needed. All services are to
/// be created once during engine start-up and re-used when needed following the singleton pattern.
pub trait Service {
    ///This method is called by the [`Scheduler`] each frame.
    fn update(&self, services: &Services);

    ///This method is used by the [`Scheduler`] during engine start-up to check if the service is
    ///set up and ready for use. This is to ensure the service is ready to handle update calls.
    fn is_ready(&self) -> bool;
}
