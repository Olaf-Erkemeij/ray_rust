use std::{rc::Rc, sync::Arc};

use camera::Camera;
use hittable::*;
use material::{Dielectric, Lambertian, Material, Metal};
use rand::Rng;
use ray::*;
use vec3::{Color, Position, Vec3};

mod hittable;
mod ray;
mod vec3;
mod camera;
mod material;

fn ppm_demo() {
    // World
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(
        Position::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(ground_material),
    )));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Position::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Position::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = rand::random::<Color>().mul(&rand::random::<Color>());
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo: Vec3<f64> = (rand::random::<Color>() + Color::new(1.0, 1.0, 1.0)) * 0.5;
                    let fuzz = rng.gen_range(0.0..0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Arc::new(Dielectric::new(1.5))
                };

                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5)) as Arc<dyn Material>;
    world.add(Box::new(Sphere::new(Position::new(0.0, 1.0, 0.0), 1.0, Arc::clone(&material1))));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))) as Arc<dyn Material>;
    world.add(Box::new(Sphere::new(Position::new(-4.0, 1.0, 0.0), 1.0, Arc::clone(&material2))));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)) as Arc<dyn Material>;
    world.add(Box::new(Sphere::new(Position::new(4.0, 1.0, 0.0), 1.0, Arc::clone(&material3))));

    // Camera
    let camera = Camera::new(
        16.0 / 9.0, 
        3840, 
        500, 
        20.0,
        Position::new(13.0, 2.0, 3.0),
        Position::new(0.0, 0.0, 0.0),
        Position::new(0.0, 1.0, 0.0), 
        0.6,
        10.0,
    );

    // Render
    camera.render(&world);
}

fn main() {
    ppm_demo();
}
