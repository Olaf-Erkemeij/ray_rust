use std::sync::{Arc, Mutex};
use std::io::Write;

use crate::{vec3::{self, Color, Direction, Position}, HittableList, Ray};

use indicatif::ProgressBar;
use rand::prelude::*;
use rayon::prelude::*;

pub struct Camera {
    img_width: i32,
    img_height: i32,
    center: Position,
    pixel00_loc: Position,
    pixel_delta_u: Position,
    pixel_delta_v: Position,
    sample_size: i32,
    max_depth: i32,
    defocus_disk_u: Direction,
    defocus_disk_v: Direction,
    defocus_angle: f64,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64, 
        img_width: i32, 
        sample_size: i32, 
        vfov: f64,
        lookfrom: Position,
        lookat: Position,
        vup: Direction,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Camera {
        let img_height = ((img_width as f64 / aspect_ratio) as i32).max(1);
        
        let center = lookfrom;
        
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (img_width as f64 / img_height as f64);

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let pixel_delta_u = viewport_u / img_width as f64;
        let pixel_delta_v = viewport_v / img_height as f64;

        let viewport_upper_left = center - (w * focus_dist) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let defocus_radius = (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        return Camera {
            img_width,
            img_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            sample_size,
            max_depth: 50,
            defocus_disk_u,
            defocus_disk_v,
            defocus_angle,
        };
    }

    pub fn render(&self, world: &HittableList) {
        let img_width = self.img_width as usize;
        let img_height = self.img_height as usize;
        let mut img = vec![Color::default(); img_width * img_height];
        let world = Arc::new(world);

        let pb = ProgressBar::new(img.len() as u64);
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("##-"),
        );

        img
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, pixel)| {
                let j = i / img_width;
                let i = i % img_width;

                let color = (0..self.sample_size)
                    .fold(Color::default(), |acc, _| {
                        let ray = self.get_ray(i as i32, j as i32);
                        acc + ray.color(&world, self.max_depth)
                    }) 
                    / self.sample_size as f64;

                *pixel = color;

                pb.inc(1);
            });

        pb.finish_with_message("Done.");

        println!("P3\n{} {}\n255\n", img_width, img_height);

        for pixel in img {
            self.write_color(pixel);
        }

        eprintln!("\rDone.");
    }

    fn write_color(&self, color: Color) {
        // Make sure color components are in the range [0, 1]
        let (r, g, b) = color.to_gamma().all();

        let ir = (256.0 * r.clamp(0.000, 0.999)) as i32;
        let ig = (256.0 * g.clamp(0.000, 0.999)) as i32;
        let ib = (256.0 * b.clamp(0.000, 0.999)) as i32;

        writeln!(std::io::stdout(), "{} {} {}", ir, ig, ib).unwrap();
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let (u, v) = (random::<f64>() - 0.5, random::<f64>() - 0.5);

        let sample = self.pixel00_loc + self.pixel_delta_u * (i as f64 + u) + self.pixel_delta_v * (j as f64 + v);

        let origin = if self.defocus_angle > 0.0 {
            let (p1, p2, _) = vec3::rand_in_unit_disk().all();

            
            self.center + self.defocus_disk_u * p1 + self.defocus_disk_v * p2
        } else {
            self.center
        };

        let direction = sample - origin;
        Ray::new(
            origin,
            direction.unit(),
        )
    }
}