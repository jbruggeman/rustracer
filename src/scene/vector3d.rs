use super::point3d::Point3D;

#[derive(Copy, Clone, Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl std::fmt::Display for Vector3D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vector: ( {}, {}, {} )", self.x, self.y, self.z)
    }
}

impl Vector3D {
    pub fn from(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D {
            x: x,
            y: y,
            z: z
        }
    }
    pub fn from_point(p: &Point3D) -> Vector3D {
        Vector3D::from(p.x, p.y, p.z)
    }

    pub fn from_points(origin: &Point3D, target: &Point3D) -> Vector3D {
        Vector3D::from(target.x - origin.x, target.y - origin.y, target.z - origin.z) 
    }

    pub fn normalize(&self) -> Vector3D {
        let scalar_value: f64 = (1.0f64.powi(2) / (self.x.powi(2) + self.y.powi(2) + self.z.powi(2))).sqrt();

        *self * scalar_value
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn cross_product(a: &Vector3D, b: &Vector3D) -> Vector3D {
        Vector3D {
            x: a.y*b.z - a.z*b.y,
            y: a.z*b.x - a.x*b.z,
            z: a.x*b.y - a.y*b.x
        }
    }

    pub fn target_point(&self) -> Point3D {
        Point3D::from(self.x, self.y, self.z)
    }
}

impl std::ops::Add<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, scalar: f64) -> Vector3D {
        Vector3D::from(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}
