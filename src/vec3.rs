// vec3.rs
use std::ops::{Add, Div, Mul, Neg, Sub};

use rand::{prelude::Distribution, Rng};

// Private Vec3 struct
#[derive(Copy, Clone, Debug)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }

    pub fn x(self) -> T {
        self.x
    }

    pub fn y(self) -> T {
        self.y
    }

    pub fn z(self) -> T {
        self.z
    }

    pub fn all(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }
}

// Implement methods for Vec3<f64>
impl Vec3<f64> {
    pub fn squared_length(self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn unit(&self) -> Vec3<f64> {
        *self / self.length()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn mul(&self, other: &Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn to_gamma(self) -> Self {
        Self::new(self.x.max(0.0).sqrt(), self.y.max(0.0).sqrt(), self.z.max(0.0).sqrt())
    }

    pub fn near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }

    pub fn reflect(&self, n: &Self) -> Self {
        *self - *n * 2.0 * self.dot(n)
    }

    pub fn refract(&self, n: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-*self).dot(n).min(1.0);
        let r_out_perp = (*self + *n * cos_theta) * etai_over_etat;
        let r_out_parallel = *n * -(1.0 - r_out_perp.squared_length()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
}

pub fn rand_unit_vec() -> Vec3<f64> {
    // Create a Uniform distribution between -1.0 and 1.0
    let between = rand::distributions::Uniform::new(-1.0, 1.0);
    let mut rng = rand::thread_rng();

    loop {
        let vec: Vec3<f64> = Vec3::new(
            rng.sample(between),
            rng.sample(between),
            rng.sample(between),
        );

        let lensq = vec.squared_length();

        if 1e-160 < lensq && lensq <= 1.0 {
            return vec / lensq.sqrt();
        }
    }
}

pub fn rand_in_unit_disk() -> Vec3<f64> {
    let between = rand::distributions::Uniform::new(-1.0, 1.0);
    let mut rng = rand::thread_rng();

    loop {
        let vec: Vec3<f64> = Vec3::new(rng.sample(between), rng.sample(between), 0.0);

        let lensq = vec.squared_length();

        if lensq <= 1.0 {
            return vec;
        }
    }
}

// Implement basic arithmetic operators generically
impl<T> Add for Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn add(self, v: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn sub(self, v: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl<T, U> Mul<U> for Vec3<T>
where
    T: Mul<U, Output = T> + Copy,
    U: Copy,
{
    type Output = Vec3<T>;

    fn mul(self, f: U) -> Vec3<T> {
        Vec3::new(self.x * f, self.y * f, self.z * f)
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn div(self, f: T) -> Vec3<T> {
        Vec3::new(self.x / f, self.y / f, self.z / f)
    }
}

impl<T> Neg for Vec3<T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn neg(self) -> Vec3<T> {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl<T> Default for Vec3<T>
where
    T: Default,
{
    fn default() -> Self {
        Vec3::new(T::default(), T::default(), T::default())
    }
}

impl Distribution<Vec3<f64>> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Vec3<f64> {
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }
}

// Public type aliases
pub type Position = Vec3<f64>;
pub type Color = Vec3<f64>;
pub type Direction = Vec3<f64>;
