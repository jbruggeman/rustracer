use serde::Deserialize;

use super::super::Color;
use super::super::point3d::Point3D;

#[derive(Deserialize, Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Plane {
    pub color: Color,
    pub p1: Point3D,
    pub p2: Point3D,
    pub p3: Point3D,
}
