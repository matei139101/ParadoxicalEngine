use crate::prelude::*;

pub struct TrackedValues {
    pub total_frames: RwLock<usize>,
    pub last_frame: RwLock<Instant>,
    pub fps: RwLock<f32>,
    pub frametime: RwLock<u128>
}

impl TrackedValues {
    pub fn new() -> TrackedValues {
        TrackedValues { 
            total_frames: 0.into(),
            last_frame: Instant::now().into(),
            fps: 0.0.into(),
            frametime: 0.into(),
        }
    }

    pub fn get_total_frames(&self) -> usize {
        if let Ok(total_frames) = self.total_frames.read() {
            *total_frames
        } else {
            todo!()
        }
    }

    pub fn set_total_frames(&self, number: usize) {
        if let Ok(mut total_frames) = self.total_frames.write() {
            *total_frames = number;
        }
    }

    pub fn get_last_frame(&self) -> Instant {
        if let Ok(last_frame) = self.last_frame.read() {
            *last_frame
        } else {
            todo!()
        }
    }

    pub fn set_last_frame(&self, time: Instant) {
        if let Ok(mut last_frame) = self.last_frame.write() {
            *last_frame = time;
        }
    }

    pub fn get_fps(&self) -> f32 {
        if let Ok(fps) = self.fps.read() {
            *fps
        } else {
            todo!()
        }
    }

    pub fn set_fps(&self, number: f32) {
        if let Ok(mut fps) = self.fps.write() {
            *fps = number;
        }
    }

    pub fn get_frametime(&self) -> u128 {
        if let Ok(frametime) = self.frametime.read() {
            *frametime
        } else {
            todo!()
        }
    }

    pub fn set_frametime(&self, number: u128) {
        if let Ok(mut frametime) = self.frametime.write() {
            *frametime = number;
        }
    }
}
