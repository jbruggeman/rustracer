pub mod raytracer {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Point {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    #[derive(Deserialize, Debug)]
    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    #[derive(Deserialize, Debug)]
    pub struct Viewport {
        pub width: u32,
        pub height: u32,
    }

    #[derive(Deserialize, Debug)]
    pub struct Camera {
        pub position: Point,
        pub target: Point,
        pub fov: f64
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
        pub viewport: Viewport,
        pub camera: Camera,
        pub objects: Objects
    }

    pub fn json_to_scene(scene_str: &String) -> serde_json::Result<Scene> {
        let v: serde_json::Value = serde_json::from_str(scene_str)?;

        let scene: Scene = (serde_json::from_value(v))?;

        Ok(scene)
    }
}