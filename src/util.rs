use bevy::prelude::*;
use rand::Rng;
use std::ops::Range;

pub fn random_vec3(x: impl RandomUnit, y: impl RandomUnit, z: impl RandomUnit) -> Vec3 {
    Vec3::new(x.get_f32(), y.get_f32(), z.get_f32())
}

pub fn random_vec3_normalized(x: impl RandomUnit, y: impl RandomUnit, z: impl RandomUnit) -> Vec3 {
    random_vec3(x.get_f32(), y.get_f32(), z.get_f32()).normalize_or_zero()
}

pub fn random_unit_vector_xz() -> Vec3 {
    random_vec3_normalized(-1.0..1.0, 0.0, -1.0..1.0)
}

pub trait RandomUnit {
    fn get_f32(&self) -> f32;
}

impl RandomUnit for f32 {
    fn get_f32(&self) -> f32 {
        *self
    }
}

impl RandomUnit for Range<f32> {
    fn get_f32(&self) -> f32 {
        rand::thread_rng().gen_range(self.clone())
    }
}
