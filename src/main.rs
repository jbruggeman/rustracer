#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};

mod scene;
mod generate;

fn main() {
    let scene_str = std::fs::read_to_string("scene.json")
        .expect("Something went wrong reading the file");

    let scene = scene::Scene::load_scene_from_json(&scene_str).expect("Error parsing JSON");
    let mut img = Image::new(scene.output_image.width, scene.output_image.height);

    for (x, y) in img.coordinates() {
        //if (x == 0 || (x == scene.output_image.width - 1)) && (y == 0 || (y == scene.output_image.height - 1)) {
        //if x == 0 && y == 0 {
            let color: scene::Color = generate::compute_pixel_from_scene(&scene, x, y);
            img.set_pixel(x, y, px!(color.r, color.g, color.b));
        //}
    }

    let _ = img.save("output.bmp");
}
