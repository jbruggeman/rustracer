use super::point3d::Point3D;
use super::vector3d::Vector3D;

#[derive(Copy, Clone, Debug)]
pub struct Ray3D {
    pub origin: Point3D,
    pub vec: Vector3D
}

impl Ray3D {
    pub fn point_on_ray(&self, length: f64) -> Point3D {
        let new_ray = Ray3D {
            origin: self.origin,
            vec: self.vec.normalize() * length
        };

        new_ray.target_point()
    }

    pub fn target_point(&self) -> Point3D {
        self.origin + self.vec.target_point()
    }
}

impl std::fmt::Display for Ray3D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Origin: ( {}, {}, {} ) Vector: ( {}, {}, {} )", 
            self.origin.x, self.origin.y, self.origin.z, self.vec.x, self.vec.y, self.vec.z)
    }
}
