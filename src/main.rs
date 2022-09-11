use std::ops::Add;
use image;
use image::{Rgb, RgbImage};
use nalgebra::{Point3, Vector, Vector3};

// TODO A List of Hittable Objects
// TODO A List of Hittable Objects
// TODO A List of Hittable Objects
// TODO A List of Hittable Objects


fn unit_vec(vec: Vector3<f64>) -> Vector3<f64> {
    let length_scalar = (vec.x.powi(2) + vec.y.powi(2) + vec.z.powi(2)).sqrt();
    vec/length_scalar
}

struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>
}

impl Ray {
    fn at(&self, t: f64) -> Vector3<f64> {
        return self.origin + (self.direction * t)
    }


    fn ray_color(&self) -> Vector3<f64> {
        let mut rec = HitRecord {
            p: Default::default(),
            normal: Default::default(),
            t: 0.0,
            front_face: false
        };
        let t = self.hit_sphere(&Vector3::new(0.0,0.0,-1.0), 0.5);
        if t > 0.0 {
            let n = unit_vec( self.at(t) - Vector3::new(0.0,0.0,-1.0));
            return 0.5 * (n.add_scalar(1.0));
        }

        let unit_dir = unit_vec(self.direction);
        let t = 0.5*(unit_dir.y+1.0);
        (1.0-t)*Vector3::new(1.0,1.0,1.0)+ t*Vector3::new(0.5,0.7,1.0)
    }

    fn hit_sphere(&self, center: &Vector3<f64>, radius: f64) -> f64{
        let oc: Vector3<f64> = ray.origin - self.center;
        let a = ray.direction.magnitude_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.magnitude_squared() - radius*radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
    }
}

struct HitRecord {
    p: Vector3<f64>,
    normal: Vector3<f64>,
    t: f64,
    front_face: bool
}

impl HitRecord {
    fn set_face_normal(&mut self, ray: &Ray, normal: Vector3<f64>) {
        self.front_face = ray.direction.dot(&normal) < 0.0;
        self.normal = if self.front_face { normal } else { -normal }
    }
}

trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64, hit_record: &mut HitRecord) -> bool;
}

struct Sphere {
    center: Vector3<f64>,
    radius: f64
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64, hit_record: &mut HitRecord) -> bool {
        let oc: Vector3<f64> = ray.origin - self.center;
        let a = ray.direction.magnitude_squared();
        let half_b = oc.dot(&ray.direction);
        let c = (oc.magnitude_squared() as f64) - radius*radius;

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

fn render_rainbow(image_buffer: &mut RgbImage) {
    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
            let r = x as f64 / (1280-1) as f64;
            let g = y  as f64 / (720-1) as f64;
            let b = 0.25;

            *pixel = image::Rgb([(255.999 * r) as u8, (255.999 * g) as u8,(255.999 * b) as u8]);
        }
}

fn render_background(image_buffer: &mut RgbImage, vpw: f64, vph: f64, focal_len: f64) {
    let origin = Vector3::zeros();
    let horizontal = Vector3::new(vpw, 0.0, 0.0);
    let vertical = Vector3::new(0.0, vph, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vector3::new(0.0, 0.0, focal_len);

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let u = (x as f64) / (1280-1) as f64;
        let v = (y as f64) / (720-1) as f64;

        let ray = Ray {origin, direction: lower_left_corner + u*horizontal + v*vertical - origin};
        let c = ray.ray_color();
        *pixel = image::Rgb([(255.999 * c.x) as u8, (255.999 * c.y) as u8,(255.999 * c.z) as u8]);
        if x == 127 && y == 574 {
            println!("{}", c)
        }
    }
}

fn main() {
    //img
    let img_w = 1280;
    let img_h = 720;
    let mut img = image::ImageBuffer::new(img_w,img_h);

    // camera
    let viewport_h = 2.0;
    let viewport_w = 16.0/9.0 * viewport_h;
    let focal_len = 1.0;

    //render_rainbow(&mut img);
    render_background(&mut img, viewport_w, viewport_h, focal_len);
    img.save_with_format("test4.1.png", image::ImageFormat::Png).unwrap();
    println!("Hello, world!");
}
