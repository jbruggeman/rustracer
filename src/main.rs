#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};
use std::time::SystemTime;

use scene::color::FloatColor;

mod scene;
mod generate;

fn main() {

    let scene_str = std::fs::read_to_string("assets/scene.json")
        .expect("Something went wrong reading the file");

    let scene = scene::Scene::load_scene_from_json(&scene_str).expect("Error parsing JSON");
    let mut img = Image::new(scene.output_image.width, scene.output_image.height);
    
    let start = SystemTime::now();

    let mut raw_color_map = vec![vec![FloatColor::black(); scene.output_image.width as usize]; scene.output_image.height as usize];

    let mut max_intensity = 0.0f64;

    for (x, y) in img.coordinates() {
        //if (x == 0 || (x == scene.output_image.width - 1)) && (y == 0 || (y == scene.output_image.height - 1)) {
        //if x == 0 && y == 0 {
       // if x == 240 && y == 240 {
            let pixel_color = generate::compute_pixel_from_scene(&scene, x, y);
        //}

        raw_color_map[x as usize][y as usize] = pixel_color;
    }

    let mut max_intensity = 0.0f64;
    for (x, y) in img.coordinates() {
        max_intensity = max_intensity.max(raw_color_map[x as usize][y as usize].r);
        max_intensity = max_intensity.max(raw_color_map[x as usize][y as usize].g);
        max_intensity = max_intensity.max(raw_color_map[x as usize][y as usize].b);
    }

    let mut intensity_coefficient = 1.0;
    if max_intensity > 1.0 {
        intensity_coefficient = 1.0 / max_intensity;
    }
    
    for (x, y) in img.coordinates() {
        let pixel = (raw_color_map[x as usize][y as usize] * intensity_coefficient).as_color();
        img.set_pixel(x, y, px!(pixel.r, pixel.g, pixel.b));
    }
    
    let elapsed = SystemTime::now().duration_since(start).expect("Fail");

    println!("Elapsed: {:?}", elapsed);

    let _ = img.save("output.bmp");
}
