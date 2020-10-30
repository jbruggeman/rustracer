use super::scene::{Scene, Color, Objects};
use super::scene::point3d::Point3D;
use super::scene::vector3d::Vector3D;
use super::scene::ray3d::Ray3D;
use super::scene::geometry::sphere::Sphere;

fn get_v_vector(roll: f64) -> Vector3D {
    // Not implemented yet...
    assert_eq!(roll, 0.0); 
    
    Vector3D::from(0.0, 1.0, 0.0)
}

fn compute_ray(scene: &Scene, x: u32, y: u32) -> Ray3D {
    // https://en.wikipedia.org/wiki/Ray_tracing_(graphics)#Calculate_rays_for_rectangular_viewport
    let t: Vector3D = Vector3D::from_point_to_point(
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

pub fn get_closest_sphere(scene: &Scene, ray: &Ray3D) -> Option<Intersection> {
    let mut closest_sphere = Option::None;

    for sphere in &scene.objects.spheres {
        match sphere.closest_point_of_intersection(&ray) {
            Some(point) => {
                match closest_sphere {
                    None => {
                        closest_sphere = Option::Some(Intersection {
                            sphere: *sphere,
                            point: point
                        });
                    },
                    Some(current_closest) => {
                        let current_point = Vector3D::from_point_to_point(&ray.origin, &current_closest.point);
                        let new_point = Vector3D::from_point_to_point(&ray.origin, &point);

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

    let light = &scene.objects.lights[0];

    let light_ray = Ray3D {
        origin: intersection.point,
        vec: Vector3D::from_point_to_point(&intersection.point, &light.position)
    };

    for sphere in &scene.objects.spheres {
        if sphere == &intersection.sphere {
            continue;
        }

        //println!("v: {:?}", light_ray);

        match sphere.closest_point_of_intersection(&light_ray) {
            None => {},
            Some(sphere_intersect) => {
                let intersect_vector = Vector3D::from_point_to_point(&light_ray.origin, &sphere_intersect);
                if intersect_vector.length() < light_ray.vec.length() {
                    light_is_blocked = true;
                }
            }
        };
    }
    
    // Hack, compute from multiple light sources.
    
    if !light_is_blocked {
        let v1 = Vector3D::from_point_to_point(&intersection.sphere.position, &intersection.point);

        let angle = Vector3D::angle(&v1, &light_ray.vec);

        let mut light_intensity : f64 = 0.0;

        if angle < (std::f64::consts::PI / 2.0) {
            light_intensity = 1.0 - (angle / (std::f64::consts::PI / 2.0));
        }

        Color {
            r: intersection.sphere.color.r,
            g: intersection.sphere.color.g,
            b: intersection.sphere.color.b
        } * light_intensity
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
