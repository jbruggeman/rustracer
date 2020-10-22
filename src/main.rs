use std::fs;


fn main() {
    let scene = fs::read_to_string("scene.json")
        .expect("Something went wrong reading the file");

    println!("Scene: {}", scene);


}
