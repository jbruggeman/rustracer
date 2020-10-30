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

    pub fn is_point_on_ray(&self, point: &Point3D) -> bool {
        // assumes that the point is on line defined by the ray and is not the origin.
        // Maybe do some checks here?
        
        let distance = Vector3D::from_point_to_point(&self.origin, &point).length();
    
        const SMALLEST_INCREMENT : f64 = 0.000001; 
        let closer_point = &self.point_on_ray(SMALLEST_INCREMENT);
        let closer_distance = Vector3D::from_point_to_point(&closer_point, &point).length();

        closer_distance < distance
    }
}

impl std::fmt::Display for Ray3D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Origin: ( {}, {}, {} ) Vector: ( {}, {}, {} )", 
            self.origin.x, self.origin.y, self.origin.z, self.vec.x, self.vec.y, self.vec.z)
    }
}
