use crate::{vec3::{self, Color}, HitRecord, Ray};

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit_record.normal() + vec3::rand_unit_vec();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal();
        }
        let scattered = Ray::new(hit_record.p(), scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray.direction().unit().reflect(&hit_record.normal());
        let scattered = Ray::new(hit_record.p(), reflected + vec3::rand_unit_vec() * self.fuzz);
        let attenuation = self.albedo;
        if scattered.direction().dot(&hit_record.normal()) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let etai_over_etat = if hit_record.front_face() {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray.direction().unit();
        let cos_theta = (-unit_direction).dot(&hit_record.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = etai_over_etat * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, etai_over_etat) > rand::random::<f64>() {
            unit_direction.reflect(&hit_record.normal())
        } else {
            unit_direction.refract(&hit_record.normal(), etai_over_etat)
        };

        let scattered = Ray::new(hit_record.p(), direction);
        let attenuation = Color::new(1.0, 1.0, 1.0);
        Some((attenuation, scattered))
    }
}