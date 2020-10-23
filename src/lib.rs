pub mod raytracer {
    use serde_json::{Value};
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct Point {
        x: f64,
        y: f64,
        z: f64,
    }

    #[derive(Deserialize, Debug)]
    struct Color {
        color: String
    }

    #[derive(Deserialize, Debug)]
    struct Viewport {
        width: u32,
        height: u32,
    }

    #[derive(Deserialize, Debug)]
    struct Camera {
        position: Point,
        target: Point,
        fov: f64
    }

    #[derive(Deserialize, Debug)]
    struct Sphere {
        color: Color,
        position: Point,
        radius: f64
    }

    #[derive(Deserialize, Debug)]
    struct Light {
        position: Point
    }

    #[derive(Deserialize, Debug)]
    struct Objects {
        spheres: Vec<Sphere>,
        lights: Vec<Light>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Scene {
        viewport: Viewport,
        camera: Camera,
        objects: Objects
    }

    pub fn json_to_scene(scene_str: &String) -> serde_json::Result<Scene> {
        let v: Value = serde_json::from_str(scene_str)?;

        let scene: Scene = (serde_json::from_value(v))?;

        Ok(scene)
    }
}