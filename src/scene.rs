use serde::Deserialize;

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "( {}, {}, {} )", self.x, self.y, self.z)
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Point;

    fn mul(self, scalar: f64) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Deserialize, Debug)]
pub struct OutputImage {
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize, Debug)]
pub struct Viewport {
    pub width: f64,
    pub height: f64,
    pub distance: f64
}

#[derive(Deserialize, Debug)]
pub struct Camera {
    pub position: Point,
    pub target: Point,
}

#[derive(Deserialize, Debug)]
pub struct Sphere {
    pub color: Color,
    pub position: Point,
    pub radius: f64
}

#[derive(Deserialize, Debug)]
pub struct Light {
    pub position: Point
}

#[derive(Deserialize, Debug)]
pub struct Objects {
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>,
}

#[derive(Deserialize, Debug)]
pub struct Scene {
    pub output_image: OutputImage,
    pub viewport: Viewport,
    pub camera: Camera,
    pub objects: Objects
}

impl Scene {
    pub fn from_json(scene_str: &String) -> serde_json::Result<Scene> {
        let v: serde_json::Value = serde_json::from_str(scene_str)?;

        let scene: Scene = (serde_json::from_value(v))?;

        Ok(scene)
    }
}

//#[derive(Copy, Clone, Debug)]
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