use super::scene::{Scene, Color};
use super::scene::point::Point;

pub struct PointVector {
    pub origin: Point,
    pub slope: Point
}

impl PointVector {
    fn from_points(origin: &Point, target: &Point) -> PointVector {
        
        PointVector {
            origin: *origin,
            slope: *target - *origin
        }
    }

    fn point_from_origin(&self, distance: &f64) -> Point {
        let scalar_value: f64 = distance.powi(2) / (self.slope.x.powi(2) + self.slope.y.powi(2) + self.slope.z.powi(2));
        let scalar_vector: Point = self.slope * scalar_value;
        
        self.origin + scalar_vector
    }
}

pub fn compute_ray(scene: &Scene, x: u32, y: u32) -> PointVector {
    let central_ray: PointVector = PointVector::from_points(
        &scene.camera.position,
        &scene.camera.target
    );

    let central_point_of_viewport: Point =  central_ray.point_from_origin(&scene.viewport.distance);

    println!("Debug: {:?}", central_point_of_viewport);

    central_ray
}

pub fn compute_pixel_from_scene(scene: &Scene, x: u32, y: u32) -> Color {
    let ray: PointVector = compute_ray(&scene, x, y);
    
    Color {
        r: 128,
        g: 0,
        b: 128
    }
}