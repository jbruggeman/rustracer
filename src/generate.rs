use super::scene::{Scene, Color, Objects, Sphere};
use super::scene::point3d::Point3D;
use super::scene::vector3d::Vector3D;
use super::scene::ray3d::Ray3D;

fn get_v_vector(roll: f64) -> Vector3D {
    // Not implemented yet...
    assert_eq!(roll, 0.0); 
    
    Vector3D::from(0.0, 1.0, 0.0)
}

fn compute_ray(scene: &Scene, x: u32, y: u32) -> Ray3D {
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

    Ray3D {
        origin: scene.camera.position,
        vec: pij
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    sphere: Sphere,
    point: Point3D
}

pub fn point_is_infront_of_camera(ray: &Ray3D, point: &Point3D) -> bool {
    // Kind of a hack, but I guess it works.
    
    let distance = Vector3D::from_points(&ray.origin, &point).length();

    const SMALLEST_INCREMENT : f64 = 0.000001;    
    let closer_point = ray.point_on_ray(SMALLEST_INCREMENT);
    let closer_distance = Vector3D::from_points(&closer_point, &point).length();

    closer_distance < distance
}

pub fn closest_point_of_intersection(sphere: &Sphere, ray: &Ray3D) -> Option<Point3D> {
    let cx = sphere.position.x;
    let cy = sphere.position.y;
    let cz = sphere.position.z;

    let p1 = ray.origin;
    let p2 = Point3D {
        x: ray.origin.x + ray.vec.x,
        y: ray.origin.y + ray.vec.y,
        z: ray.origin.z + ray.vec.z
    };

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
        return Option::None;
    }

    let t1 = ( -b - d.sqrt() ) / ( 2.0 * a );
    let solution1 = Point3D {
        x: p1.x * ( 1.0 - t1 ) + t1 * p2.x,
        y: p1.y * ( 1.0 - t1 ) + t1 * p2.y,
        z: p1.z * ( 1.0 - t1 ) + t1 * p2.z
    };
        
    if d == 0.0 {
        if point_is_infront_of_camera(&ray, &solution1) {
            return Option::Some(solution1);
        } else {
            return Option::None;
        }
    } 

    let t2 = ( -b + d.sqrt() ) / ( 2.0 * a );
    let solution2 = Point3D {
        x: p1.x * ( 1.0 - t2 ) + t2 * p2.x,
        y: p1.y * ( 1.0 - t2 ) + t2 * p2.y,
        z: p1.z * ( 1.0 - t2 ) + t2 * p2.z
    };

    let sol1_vec3d = Vector3D::from_points(&ray.origin, &solution1);
    let sol2_vec3d = Vector3D::from_points(&ray.origin, &solution2);

    let solution_1_visible = point_is_infront_of_camera(&ray, &solution1);
    let solution_2_visible = point_is_infront_of_camera(&ray, &solution2);

    if !solution_1_visible && !solution_2_visible {
        return Option::None;
    }

    if solution_1_visible && solution_2_visible {
        if sol1_vec3d.length() < sol2_vec3d.length() {
            return Option::Some(solution1);
        } else {
            return Option::Some(solution2);
        }
    }

    // What to do here. We're in the middle of a sphere...
    panic!("Camera is in a sphere! I don't know what to do :(");
}

pub fn get_closest_sphere(scene: &Scene, ray: &Ray3D) -> Option<Intersection> {
    let mut closest_sphere = Option::None;

    for sphere in &scene.objects.spheres {
        match closest_point_of_intersection(&sphere, &ray) {
            Some(point) => {
                match closest_sphere {
                    None => {
                        closest_sphere = Option::Some(Intersection {
                            sphere: *sphere,
                            point: point
                        });
                    },
                    Some(current_closest) => {
                        let current_point = Vector3D::from_points(&ray.origin, &current_closest.point);
                        let new_point = Vector3D::from_points(&ray.origin, &point);

                        if new_point.length() < current_point.length() {
                            closest_sphere = Option::Some(Intersection {
                                sphere: *sphere,
                                point: point
                            });
                        }
                    }
                }
            },
            None => ()
        }
    }

    closest_sphere
}  

pub fn compute_color(scene: &Scene, intersection: &Intersection) -> Color {
    let mut light_is_blocked = false;

    for light in &scene.objects.lights {
        for sphere in &scene.objects.spheres {
            if sphere == &intersection.sphere {
                continue;
            }

            let line = Ray3D {
                origin: intersection.point,
                vec: Vector3D::from_points(&light.position, &intersection.point)
            };

            println!("v: {:?}", line.vec);

            match closest_point_of_intersection(&sphere, &line) {
                None => {},
                Some(intersect) => {
                    let intersect_vector = Vector3D::from_points(&intersect, &intersection.point);
                    if intersect_vector.length() < line.vec.length() {
                        light_is_blocked = true;
                    }
                }
            };
        }
    }
    
    // Hack, compute from multiple light sources.
    if !light_is_blocked {
        Color {
            r: intersection.sphere.color.r,
            g: intersection.sphere.color.g,
            b: intersection.sphere.color.b
        }
    } else {
        Color::black()
    }
}

pub fn compute_pixel_from_scene(scene: &Scene, x: u32, y: u32) -> Color {
    let ray: Ray3D = compute_ray(&scene, x, y);
    //println!("Debug: {:?}", ray);
    
    match get_closest_sphere(&scene, &ray) {
        Some(intersection) => {
            compute_color(&scene, &intersection)
        },
        None => {
            Color::black()
        }
    }
}
