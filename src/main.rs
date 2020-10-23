use std::fs;
use raytracer::raytracer::json_to_scene;

fn main() {
    let scene_str = fs::read_to_string("scene.json")
        .expect("Something went wrong reading the file");

    println!("Scene: {}", scene_str);

    let scene = json_to_scene(&scene_str);
}
