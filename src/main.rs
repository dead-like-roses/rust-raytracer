mod ray;
mod hit_record;
mod hittable;

use std::ops::Add;
use image;
use image::{Rgb, RgbImage};
use nalgebra::{Point3, Vector, Vector3};
use crate::hittable::{HittableList, Sphere};
use crate::ray::Ray;

fn render_background(image_buffer: &mut RgbImage, vpw: f64, vph: f64, focal_len: f64) {
    let origin = Vector3::new(0.0,0.0,0.0);
    let horizontal = Vector3::new(vpw, 0.0, 0.0);
    let vertical = Vector3::new(0.0, vph, 0.0);
    let lower_left_corner = origin - (horizontal/2.0) - (vertical/2.0) - Vector3::new(0.0, 0.0, focal_len);
    println!("{}",horizontal);
    println!("{}",vertical);
    
    let mut world = HittableList::new();
    world.add(Box::new(Sphere { center: Vector3::new(0.0,0.0,-1.0), radius: 0.5 }));
    world.add(Box::new(Sphere { center: Vector3::new(0.0,-100.5,-1.0), radius: 100.0 }));

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let tempy = (720-1)-y;
        let u = (x as f64) / (1280-1) as f64;
        let v = (tempy as f64) / (720-1) as f64;

        let ray = Ray {origin, direction: lower_left_corner + u*horizontal + v*vertical - origin};
        let c = ray.ray_color(&world);
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
    let viewport_w = (1280.0/720.0) * viewport_h;
    let focal_len = 1.0;

    //render_rainbow(&mut img);
    render_background(&mut img, viewport_w, viewport_h, focal_len);
    img.save_with_format("test4.2.png", image::ImageFormat::Png).unwrap();
    println!("Hello, world!");
}
