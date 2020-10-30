use serde::Deserialize;

pub mod point3d;
pub mod vector3d;
pub mod ray3d;
pub mod geometry;

use geometry::sphere::Sphere;
use point3d::Point3D;

#[derive(Deserialize, Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn black() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0
        }
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, scalar: f64) -> Color {
        Color {
            r: (self.r as f64 * scalar) as u8,
            g: (self.g as f64 * scalar) as u8,
            b: (self.b as f64 * scalar) as u8,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct OutputImage {
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize, Debug)]
pub struct Camera {
    pub position: Point3D,
    pub target: Point3D,
    pub fov: f64
}

#[derive(Deserialize, Debug)]
pub struct Light {
    pub position: Point3D
}

#[derive(Deserialize, Debug)]
pub struct Objects {
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>,
}

#[derive(Deserialize, Debug)]
pub struct Scene {
    pub output_image: OutputImage,
    pub camera: Camera,
    pub objects: Objects
}

impl Scene {
    pub fn load_scene_from_json(scene_str: &String) -> serde_json::Result<Scene> {
        let v: serde_json::Value = serde_json::from_str(scene_str)?;

        let scene: Scene = (serde_json::from_value(v))?;

        Ok(scene)
    }
}
