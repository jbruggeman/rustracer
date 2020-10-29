use super::scene::{Scene, Color, Objects, Sphere};
use super::scene::point::Point;
use super::scene::vector3d::Vector3D;
use super::scene::line3d::Line3D;

fn get_v_vector(roll: f64) -> Vector3D {
    // Not implemented yet...
    assert_eq!(roll, 0.0); 
    
    Vector3D::from(0.0, 1.0, 0.0)
}

fn compute_ray(scene: &Scene, x: u32, y: u32) -> Line3D {
    // https://en.wikipedia.org/wiki/Ray_tracing_(graphics)#Calculate_rays_for_rectangular_viewport
    let t: Vector3D = Vector3D::from_points(
        &scene.camera.position,
        &scene.camera.target
    );

    let v: Vector3D = get_v_vector(0.0); 
    let b: Vector3D = Vector3D::cross_product(&t, &v);

    let tn = t.normalize();
    let vn = v.normalize();
    let bn = Vector3D::cross_product(&tn, &vn);

    let fov_radians = scene.camera.fov.to_radians();

    let gx = (fov_radians / 2.0).tan();
    let gy = gx * ((scene.output_image.height - 1) as f64) / ((scene.output_image.width - 1) as f64);

    let qx = bn * (2.0 * gx / ((scene.output_image.width - 1) as f64));
    let qy = vn * (2.0 * gy / ((scene.output_image.height - 1) as f64));

    let p1m = tn - bn*gx - vn*gy;

    let pij = p1m + qx * (x as f64) + qy * (y as f64);

    Line3D {
        origin: scene.camera.position,
        vec: pij
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

pub fn num_points_of_intersection(sphere: &Sphere, ray: &Line3D) -> PointsOfIntersection {
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

pub fn get_closest_sphere(scene: &Scene, ray: &Line3D) -> Option<Intersection> {
    let mut ret = Option::None;

    // Hack
    for sphere in &scene.objects.spheres {
        match num_points_of_intersection(&sphere, &ray) {
            PointsOfIntersection::One | PointsOfIntersection::Two => {
                ret = Option::Some(Intersection {
                    sphere: *sphere,
                    point: Point::zero()
                });
            },
            _ => ()
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
    let ray: Line3D = compute_ray(&scene, x, y);
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
