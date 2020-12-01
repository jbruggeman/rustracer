pub mod sphere;
pub mod plane;

use super::point3d::Point3D;
use super::ray3d::Ray3D;

pub trait Intersect {
    fn intersect(&self, ray: &Ray3D) -> Vec<Point3D>;
}