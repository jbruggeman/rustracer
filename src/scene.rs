use serde::Deserialize;

pub mod point3d;
pub mod vector3d;
pub mod ray3d;
pub mod geometry;
pub mod color;

use geometry::sphere::Sphere;
use geometry::plane::Plane;
use point3d::Point3D;

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
pub struct Objects {
    #[serde(default = "Vec::new")]
    pub spheres: Vec<Sphere>,
    
    #[serde(default = "Vec::new")]
    pub plane: Vec<Plane>
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
