use std::{ops::Range, rc::Rc, sync::Arc};

use crate::{
    material::Material, vec3::{Direction, Position}, Ray
};

pub type Interval = Range<f64>;

pub struct HitRecord {
    p: Position,
    normal: Direction,
    t: f64,
    front_face: bool,
    material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(p: Position, normal: Direction, t: f64, front_face: bool, material: Arc<dyn Material>) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }

    pub fn p(&self) -> Position {
        self.p
    }

    pub fn normal(&self) -> Direction {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Direction) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    pub fn material(&self) -> Arc<dyn Material> {
        Arc::clone(&self.material)
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}

// Create a sphere that implements the Hittable trait
pub struct Sphere {
    center: Position,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Position, radius: f64, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius: radius.max(0.0),
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().squared_length();
        let half_b = oc.dot(&ray.direction());
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if !interval.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !interval.contains(&root) {
                return None;
            }
        }

        let outward_normal = (ray.at(root) - self.center) / self.radius;
        let mut res = HitRecord::new(
            ray.at(root),
            outward_normal,
            root,
            false,
            Arc::clone(&self.material),
        );

        res.set_face_normal(ray, outward_normal);

        Some(res)
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let mut closest_so_far = interval.end;
        let mut res: Option<HitRecord> = None;

        for object in self.objects.iter() {
            if let Some(hit_record) = object.hit(ray, interval.start..closest_so_far) {
                closest_so_far = hit_record.t();
                res = Some(hit_record);
            }
        }

        res
    }
}
