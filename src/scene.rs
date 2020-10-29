use serde::Deserialize;

pub mod point;
pub mod vector3d;
pub mod line3d;

use point::Point;

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
pub struct Camera {
    pub position: Point,
    pub target: Point,
    pub fov: f64
}

#[derive(Deserialize, Copy, Clone, Debug)]
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
