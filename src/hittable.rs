use nalgebra::Vector3;
use crate::hit_record::HitRecord;
use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64, hit_record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64, hit_record: &mut HitRecord) -> bool {
        let oc: Vector3<f64> = ray.origin - self.center;
        let a = ray.direction.magnitude_squared();
        let half_b = oc.dot(&ray.direction);
        let c = (oc.magnitude_squared() as f64) - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd)/a;
        if root < tmin || root > tmax {
            root = -(half_b + sqrtd) /a;
            if root < tmin || tmax < root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = ray.at(hit_record.t);
        let outward_normal= (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);
        true
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64, hit_record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest = tmax;

        for o in &self.objects {
            if o.hit(ray, tmin, closest, hit_record) {
                hit_anything = true;
                closest = hit_record.t;
            }
        }

        hit_anything
    }
}

impl HittableList {
    pub fn new() -> Self {
        return HittableList {objects: Vec::new()}
    }

    pub fn clear(&mut self) {
        self.objects = Vec::new();
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.objects.push(hittable);
    }
}
