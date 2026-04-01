pub trait Service {
    fn update(&self);
    fn is_ready(&self) -> bool;
}
