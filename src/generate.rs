use super::scene::{Scene, Color};
use super::scene::point::Point;

#[derive(Copy, Clone, Debug)]
pub struct PointVector {
    pub origin: Point,
    pub vec: Point
}

impl std::fmt::Display for PointVector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Origin: ( {}, {}, {} ) Vector: ( {}, {}, {} )", 
            self.origin.x, self.origin.y, self.origin.z, self.vec.x, self.vec.y, self.vec.z)
    }
}

impl PointVector {
    fn from_points(origin: &Point, target: &Point) -> PointVector {
        let v : Point = *target - *origin;

        PointVector {
            origin: *origin,
            vec: v
        }
    }

    fn to_unit_vector(&self) -> PointVector {
        let scalar_value: f64 = (1.0f64.powi(2) / (self.vec.x.powi(2) + self.vec.y.powi(2) + self.vec.z.powi(2))).sqrt();

        PointVector {
            origin: self.origin,
            vec: self.vec * scalar_value
        }
    }

    fn length(&self) -> f64 {
        (self.vec.x.powi(2) + self.vec.y.powi(2) + self.vec.z.powi(2)).sqrt()
    }
}

impl std::ops::Mul<f64> for PointVector {
    type Output = PointVector;

    fn mul(self, scalar: f64) -> PointVector {
        PointVector {
            origin: self.origin,
            vec: Point {
                x: self.vec.x * scalar,
                y: self.vec.y * scalar,
                z: self.vec.z * scalar,
            }
        }
    }
}

fn raw_viewport_vector_for_pixel(scene: &Scene, x: u32, y:u32) -> PointVector {
    let viewport_vector : Point = Point {
        x: scene.viewport.width * ((x as f64) / ((scene.output_image.width - 1) as f64)) - (scene.viewport.width / 2.0),
        y: scene.viewport.height * ((y as f64) / ((scene.output_image.height - 1) as f64)) - (scene.viewport.height / 2.0),
        z: scene.viewport.distance
    };

    PointVector {
        origin: Point {
            x: 0.0,
            y: 0.0,
            z: 0.0
        },
        vec: viewport_vector
    }
}

fn raw_viewport_vector_central_unit_vector() -> PointVector {
    PointVector {
        origin: Point {
            x: 0.0,
            y: 0.0,
            z: 0.0
        },
        vec: Point {
            x: 0.0,
            y: 0.0,
            z: 1.0
        }
    }
}


fn compute_ray(scene: &Scene, x: u32, y: u32) -> PointVector {
    let raw_viewport_vector: PointVector = raw_viewport_vector_for_pixel(&scene, x, y);
    println!("Raw Viewport Vector: {:?}", raw_viewport_vector);

    println!("Raw viewport unit vector {:?}", raw_viewport_vector_central_unit_vector());

    let central_ray_vector: PointVector = PointVector::from_points(
        &scene.camera.position,
        &scene.camera.target
    );
    println!("central_ray_vector: {:?}", central_ray_vector);

    let central_ray_unit_vector: PointVector = central_ray_vector.to_unit_vector();
    println!("central_ray_unit_vector: {:?}", central_ray_unit_vector);

    let rotation_vector : Point = central_ray_unit_vector.vec - raw_viewport_vector_central_unit_vector().vec;
    println!("rotation_vector: {:?}", rotation_vector);

    let pixel_unit_vector = raw_viewport_vector.to_unit_vector().vec + rotation_vector;
    println!("pixel_unit_vector: {:?}", pixel_unit_vector);

    PointVector {
        origin: scene.camera.position,
        vec: pixel_unit_vector * raw_viewport_vector.length()
    }
}

pub fn compute_pixel_from_scene(scene: &Scene, x: u32, y: u32) -> Color {
    let ray: PointVector = compute_ray(&scene, x, y);
    
    println!("Debug: {:?}", ray);
    
    Color {
        r: 128,
        g: 0,
        b: 128
    }
}