use super::point::Point;
use super::vector3d::Vector3D;

#[derive(Copy, Clone, Debug)]
pub struct Line3D {
    pub origin: Point,
    pub vec: Vector3D
}

impl std::fmt::Display for Line3D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Origin: ( {}, {}, {} ) Vector: ( {}, {}, {} )", 
            self.origin.x, self.origin.y, self.origin.z, self.vec.x, self.vec.y, self.vec.z)
    }
}
