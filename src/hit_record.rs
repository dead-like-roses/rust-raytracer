use nalgebra::Vector3;
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            p: Default::default(),
            normal: Default::default(),
            t: 0.0,
            front_face: false
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, normal: Vector3<f64>) {
        self.front_face = ray.direction.dot(&normal) < 0.0;
        self.normal = if self.front_face { normal } else { -normal }
    }
}