use super::scene::{Scene, Color};
use super::scene::point::Point;

pub struct PointVector {
    pub origin: Point,
    pub unit_vector: Point
}

impl PointVector {
    fn from_points(origin: &Point, target: &Point) -> PointVector {
        let v : Point = *target - *origin;

        let scalar_value: f64 = (1.0f64.powi(2) / (v.x.powi(2) + v.y.powi(2) + v.z.powi(2))).sqrt();

        PointVector {
            origin: *origin,
            unit_vector: v * scalar_value
        }
    }

    fn point_from_origin(&self, distance: &f64) -> Point {
        self.origin + self.unit_vector * (*distance)
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