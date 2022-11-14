use crate::material::{Material, Lambertian};
use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};

use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;

        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64, front_face: bool, material: Rc<dyn Material>) -> Self {
        Self {
            p,
            normal,
            t,
            front_face,
            material
        }
    }

    pub fn default() -> Self {
        Self::new(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            0.0,
            false,
            Rc::new(Lambertian::default())
        )
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
