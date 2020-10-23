mod scene;

fn main() {
    let scene_str = std::fs::read_to_string("scene.json")
        .expect("Something went wrong reading the file");

    println!("Scene: {}", scene_str);

    let scene = scene::Scene::from_json(&scene_str).expect("Error parsing JSON");

    println!("Scene width: {}", scene.viewport.width);
}
