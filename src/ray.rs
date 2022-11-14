use crate::vec3::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3
}

impl Default for Ray {
    fn default() -> Self {
        Self::new(Point3::default(), Vec3::default())
    }
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction
        }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
