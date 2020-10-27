use super::scene::{Scene, Color};
use super::scene::point::Point;

pub struct PointVector {
    pub origin: Point,
    pub vec: Point
}

impl PointVector {
    fn to_unit_vector(origin: &Point, target: &Point) -> PointVector {
        let v : Point = *target - *origin;

        let scalar_value: f64 = (1.0f64.powi(2) / (v.x.powi(2) + v.y.powi(2) + v.z.powi(2))).sqrt();

        PointVector {
            origin: *origin,
            vec: v * scalar_value
        }
    }

    fn point_from_origin(&self, distance: &f64) -> Point {
        self.origin + self.vec * (*distance)
    }
}

pub fn compute_gaze_offset(scene: &Scene, x: u32, y:u32) -> Point {
    let viewport_vector : Point = Point {
        x: scene.viewport.width * ((x as f64) / ((scene.output_image.width - 1) as f64)) - (scene.viewport.width / 2.0),
        y: scene.viewport.height * ((y as f64) / ((scene.output_image.height - 1) as f64)) - (scene.viewport.height / 2.0),
        z: scene.viewport.distance
    };

    viewport_vector
}

pub fn compute_ray(scene: &Scene, x: u32, y: u32) -> PointVector {
    let central_ray_unit_vector: PointVector = PointVector::to_unit_vector(
        &scene.camera.position,
        &scene.camera.target
    );

    let central_point_of_viewport: Point =  central_ray_unit_vector.point_from_origin(&scene.viewport.distance);

    println!("Debug: {:?}", central_point_of_viewport);

    let gaze_offset: Point = compute_gaze_offset(&scene, x, y);
    println!("Gaze offset: {:?}", gaze_offset);

    central_ray_unit_vector
}

pub fn compute_pixel_from_scene(scene: &Scene, x: u32, y: u32) -> Color {
    let ray: PointVector = compute_ray(&scene, x, y);
    
    Color {
        r: 128,
        g: 0,
        b: 128
    }
}