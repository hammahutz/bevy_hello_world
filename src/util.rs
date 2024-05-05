use std::{ops::Range, vec};

use bevy::prelude::*;
use rand::Rng;

pub enum VectorType {
    Scalar, //TODO Ta reda på rätt namn
    Unit,
}

pub fn random_2d_unit_vector() -> Vec3 {
    let vector_type = VectorType::Unit;
    let x = Some(-1.0..1.0);
    let y = None;
    let z = Some(-1.0..1.0);

    random_vec3(vector_type, x, y, z)
}
pub fn random_vec3(
    vector_type: VectorType,
    x: Option<Range<f32>>,
    y: Option<Range<f32>>,
    z: Option<Range<f32>>,
) -> Vec3 {
    let mut rng = rand::thread_rng();

    let x = match x {
        Some(_) => rng.gen_range(x.unwrap()),
        None => 0.0,
    };
    let y = match y {
        Some(_) => rng.gen_range(y.unwrap()),
        None => 0.0,
    };
    let z = match z {
        Some(_) => rng.gen_range(z.unwrap()),
        None => 0.0,
    };

    match vector_type {
        VectorType::Scalar => Vec3::new(x, y, z),
        VectorType::Unit => Vec3::new(x, y, z).normalize_or_zero(),
    }
}

pub fn random_unit_float() -> f32 {
    rand::thread_rng().gen_range(-1.0..1.0)
}
