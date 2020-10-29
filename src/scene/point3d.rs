use serde::Deserialize;

#[derive(Deserialize, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn zero() -> Point3D {
        Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
    pub fn from(x: f64, y: f64, z: f64) -> Point3D {
        Point3D {
            x: x,
            y: y,
            z: z
        }
    }
}

impl std::fmt::Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "( {}, {}, {} )", self.x, self.y, self.z)
    }
}

impl std::ops::Add<Point3D> for Point3D {
    type Output = Point3D;

    fn add(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub<Point3D> for Point3D {
    type Output = Point3D;

    fn sub(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<f64> for Point3D {
    type Output = Point3D;

    fn mul(self, scalar: f64) -> Point3D {
        Point3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}
