use nalgebra::Vector3;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>
}

impl Ray {
    pub fn at(&self, t: f64) -> Vector3<f64> {
        return self.origin + (self.direction * t)
    }

    pub fn ray_color(&self, world: &dyn Hittable) -> Vector3<f64> {
        let mut rec = HitRecord::new();
        if world.hit(self, 0.0, f64::INFINITY, &mut rec) {
            return 0.5 * (rec.normal + Vector3::new(1.0,1.0,1.0))
        }
        let unit_dir = self.direction.normalize();
        let t = 0.5*(unit_dir.y+1.0);
        (1.0-t)*Vector3::new(1.0,1.0,1.0)+ t*Vector3::new(0.5,0.7,1.0)
    }

}