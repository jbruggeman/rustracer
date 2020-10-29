use super::scene::{Scene, Color, Objects, Sphere};
use super::scene::point::Point;
use super::scene::pointvector::PointVector;

fn raw_viewport_vector_for_pixel(scene: &Scene, x: u32, y:u32) -> PointVector {
    let viewport_vector : Point = Point {
        x: scene.viewport.width * ((x as f64) / ((scene.output_image.width - 1) as f64)) - (scene.viewport.width / 2.0),
        y: (scene.viewport.height / 2.0) - scene.viewport.height * ((y as f64) / ((scene.output_image.height - 1) as f64)),
        z: scene.viewport.distance
    };

    PointVector {
        origin: Point::zero(),
        vec: viewport_vector
    }
}

fn raw_viewport_vector_central_unit_vector() -> PointVector {
    PointVector {
        origin: Point::zero(),
        vec: Point::from(0.0, 0.0, 1.0)
    }
}


fn compute_ray(scene: &Scene, x: u32, y: u32) -> PointVector {
    let raw_viewport_vector: PointVector = raw_viewport_vector_for_pixel(&scene, x, y);

    let central_ray_vector: PointVector = PointVector::from_points(
        &scene.camera.position,
        &scene.camera.target
    );

    let central_ray_unit_vector: PointVector = central_ray_vector.to_unit_vector();

    let rotation_vector : Point = central_ray_unit_vector.vec - raw_viewport_vector_central_unit_vector().vec;

    let pixel_vector = raw_viewport_vector.to_unit_vector().vec + rotation_vector;
    let pixel_unit_vector = PointVector::from_vector(&pixel_vector).to_unit_vector();

    /*println!("Raw Viewport Vector: {:?}", raw_viewport_vector);
    println!("Raw Viewport Unit Vector: {:?}", raw_viewport_vector.to_unit_vector().vec);
    println!("Central Raw viewport unit vector {:?}", raw_viewport_vector_central_unit_vector());
    println!("central_ray_vector: {:?}", central_ray_vector);
    println!("central_ray_unit_vector: {:?}", central_ray_unit_vector);
    println!("rotation_vector: {:?}", rotation_vector);
    println!("pixel_vector: {:?}", pixel_unit_vector);*/
    
    PointVector {
        origin: scene.camera.position,
        vec: pixel_unit_vector.vec * raw_viewport_vector.length()
    }
}

pub struct Intersection {
    sphere: Sphere,
    point: Point
}

pub enum PointsOfIntersection {
    None,
    One,
    Two
}

pub fn num_points_of_intersection(sphere: &Sphere, ray: &PointVector) -> PointsOfIntersection {
    let cx = sphere.position.x;
    let cy = sphere.position.y;
    let cz = sphere.position.z;

    let p1 = ray.origin;

    let px = p1.x;
    let py = p1.y;
    let pz = p1.z;
    
    let vx = ray.vec.x;
    let vy = ray.vec.y;
    let vz = ray.vec.z;
        
    let r = sphere.radius;
    
    let a : f64 =  vx * vx + vy * vy + vz * vz;
    let b : f64 = 2.0 * (px * vx + py * vy + pz * vz - vx * cx - vy * cy - vz * cz);
    let c : f64 = px * px - 2.0 * px * cx + cx * cx + py * py - 2.0 * py * cy + cy * cy + pz * pz - 2.0 * pz * cz + cz * cz - r * r;

    let d = b * b - 4.0 * a * c;
 
    if d < 0.0 {
        PointsOfIntersection::None
    } else if d == 0.0 {
        PointsOfIntersection::One
    } else {
        PointsOfIntersection::Two
    }
}

pub fn get_closest_sphere(scene: &Scene, ray: &PointVector) -> Option<Intersection> {
    let mut ret = Option::None;

    // Hack
    for sphere in &scene.objects.spheres {
        match num_points_of_intersection(&sphere, &ray) {
            PointsOfIntersection::None => (),
            _ => {
                ret = Option::Some(Intersection {
                    sphere: *sphere,
                    point: Point::zero()
                });
            }
        }
    }

    ret
}  

pub fn compute_color(scene: &Scene, intersection: &Intersection) -> Color {
    Color {
        r: intersection.sphere.color.r,
        g: intersection.sphere.color.g,
        b: intersection.sphere.color.b
    }
}

pub fn compute_pixel_from_scene(scene: &Scene, x: u32, y: u32) -> Color {
    let ray: PointVector = compute_ray(&scene, x, y);
    //println!("Debug: {:?}", ray);
    
    match get_closest_sphere(&scene, &ray) {
        Some(intersection) => {
            compute_color(&scene, &intersection)
        },
        None => {
            Color {
                r: 0,
                g: 0,
                b: 0
            } 
        }
    }
}
