use std::{ops::{self, Neg}, cmp::min};
use rand::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl Default for Vec3 {
    fn default() -> Self {
        Self {x: 0.0, y: 0.0, z: 0.0}
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x, y, z
        }
    }

    pub fn random() -> Self {
        Self::random_range(0.0, 1.0)
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = thread_rng();

        Self::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1.0, 1.0);

            if p.length_squared() >= 1.0 {
                continue;
            }

            return p;
        }
    }

    pub fn random_unit_vector() ->  Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_hemisphere(normal: Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();

        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn length(self) -> f64 {
        self
            .length_squared()
            .sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn near_zero(self) -> bool {
        let s = 1e-8;

        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(self, normal: Self) -> Self {
        self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(self, n: Self, etai_over_etat: f64) -> Self {
        let cos_theta = self.neg().dot(n).min(1.0);

        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = (1.0 - r_out_perp.length_squared()).abs().sqrt().neg() * n;

        r_out_perp + r_out_parallel
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = thread_rng();
        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);

            if p.length_squared() >= 1.0 { continue }

            return p;
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y, 
            z: -self.z
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3; 

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3; 

    fn mul(self, scalar: f64) -> Self::Output {
        scalar * self
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3; 

    fn mul(self, v: Vec3) -> Self::Output {
        Self::Output {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3; 

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3; 

    fn div(self, v: Vec3) -> Self::Output {
        (1.0/self) * v
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3; 

    fn div(self, scalar: f64) -> Self::Output {
        self * (1.0 / scalar)
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3; 

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_neg_ops() {
        let vec = Vec3::new(1.0, -3.0, 0.0);

        let neg_vec = -vec;

        assert_eq!(neg_vec.x, -vec.x);
        assert_eq!(neg_vec.y, -vec.y);
        assert_eq!(neg_vec.z, -vec.z);
    }

    #[test]
    fn vec3_add_ops() {
        let vec = Vec3::new(1.0, -3.0, 0.0);
        let other_vec = Vec3::new(4.0, -5.0, -1.0);

        let vec = vec + other_vec;

        assert_eq!(vec.x, 5.0);
        assert_eq!(vec.y, -8.0);
        assert_eq!(vec.z, -1.0);
    }

    #[test]
    fn vec3_sub_ops() {
        let vec = Vec3::new(1.0, -3.0, 0.0);
        let other_vec = Vec3::new(4.0, -5.0, -1.0);

        let vec = vec - other_vec;

        assert_eq!(vec.x, -3.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 1.0);
    }

    #[test]
    fn vec3_mul_scalar_ops() {
        let vec = Vec3::new(1.0, -3.0, 0.0);
        let scalar = 2.0;

        let vec = vec * scalar;

        assert_eq!(vec.x, 2.0);
        assert_eq!(vec.y, -6.0);
        assert_eq!(vec.z, 0.0);

        assert_eq!(vec * scalar, scalar * vec);
    }


    #[test]
    fn vec3_div_scalar_ops() {
        let vec = Vec3::new(1.0, -3.0, 0.0);
        let scalar = 2.0;

        let vec = vec / scalar;

        assert_eq!(vec.x, 0.5);
        assert_eq!(vec.y, -1.5);
        assert_eq!(vec.z, 0.0);
    }


    #[test]
    fn vec3_dot_product() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let other = Vec3::new(1.0, 3.0, 3.0);

        assert_eq!(vec.dot(other), 16.0);
    }

    #[test]
    fn vec3_cross() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let other = Vec3::new(1.0, 3.0, 3.0);

        assert_eq!(vec.cross(other), Vec3::new(-3.0, 0.0, 1.0));
    }

    #[test]
    fn vec3_unit_vector() {
        let vec = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(vec.unit_vector(), Vec3::new(0.2672612419124244, 0.5345224838248488, 0.8017837257372732))
    }
}

