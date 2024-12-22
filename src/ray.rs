use crate::{
    hittable::Hittable,
    vec3::{Color, Direction, Position}, HittableList,
};

pub struct Ray {
    origin: Position,
    direction: Direction,
}

impl Ray {
    pub fn new(origin: Position, direction: Direction) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Position {
        self.origin + self.direction * t
    }

    pub fn origin(&self) -> Position {
        self.origin
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn color(&self, world: &HittableList, depth: i32) -> Color {
        if depth <= 0 {
            return Color::default();
        }

        if let Some(rec) = world.hit(self, 0.001..f64::INFINITY) {
            if let Some((attenuation, scattered)) = rec.material().scatter(self, &rec) {
                return attenuation.mul(&scattered.color(world, depth - 1));
            }

            return Color::default();
        }

        let unit_dir = self.direction.unit();
        let a = 0.5 * (unit_dir.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
