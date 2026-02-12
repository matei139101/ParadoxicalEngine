use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    //pub size: Vec3,
}

impl Transform {
    pub fn new(position: Vec3, rotation: Vec3 /* size: Vec3 */) -> Self {
        Transform { position, rotation }
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn get_rotation(&self) -> Vec3 {
        self.rotation
    }

    /*
    pub fn get_size(&self) -> Vec3 {
        return self.size;
    }
    */

    pub fn right(&self) -> Vec3 {
        Vec3::new(
            self.rotation.y.cos() * self.rotation.z.cos(),
            self.rotation.z.sin(),
            self.rotation.y.sin() * self.rotation.z.cos(),
        )
    }

    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            self.rotation.y.sin(),
            -self.rotation.x.sin(),
            -self.rotation.y.cos(),
        )
    }

    pub fn up(&self) -> Vec3 {
        Vec3::new(
            self.rotation.x.sin() * self.rotation.y.sin() * self.rotation.z.cos()
                - self.rotation.x.cos() * self.rotation.z.sin(),
            self.rotation.x.cos() * self.rotation.z.cos()
                + self.rotation.x.sin() * self.rotation.y.sin() * self.rotation.z.sin(),
            -self.rotation.x.sin() * self.rotation.y.cos(),
        )
    }

    pub fn offset_y_rotation(&mut self, amount: f64) {
        self.rotation.y += amount as f32;
    }

    pub fn offset_x_rotation(&mut self, amount: f64) {
        self.rotation.x += amount as f32;
    }
}
