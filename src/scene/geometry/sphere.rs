use serde::Deserialize;

use super::super::Color;
use super::super::point3d::Point3D;
use super::super::ray3d::Ray3D;
use super::super::vector3d::Vector3D;

#[derive(Deserialize, Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Sphere {
    pub color: Color,
    pub position: Point3D,
    pub radius: f64
}

impl Sphere {
    pub fn points_of_intersection(&self, ray: &Ray3D) -> Vec<Point3D> {
        let cx = self.position.x;
        let cy = self.position.y;
        let cz = self.position.z;

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
            
        let r = self.radius;
        
        let a : f64 =  vx * vx + vy * vy + vz * vz;
        let b : f64 = 2.0 * (px * vx + py * vy + pz * vz - vx * cx - vy * cy - vz * cz);
        let c : f64 = px * px - 2.0 * px * cx + cx * cx + py * py - 2.0 * py * cy + cy * cy + pz * pz - 2.0 * pz * cz + cz * cz - r * r;

        let d = b * b - 4.0 * a * c;
    
        if d < 0.0 {
            return vec![];
        }

        let t1 = ( -b - d.sqrt() ) / ( 2.0 * a );
        let solution1 = Point3D {
            x: p1.x * ( 1.0 - t1 ) + t1 * p2.x,
            y: p1.y * ( 1.0 - t1 ) + t1 * p2.y,
            z: p1.z * ( 1.0 - t1 ) + t1 * p2.z
        };
            
        if d == 0.0 {
            if ray.is_point_on_ray(&solution1) {
                return vec![solution1];
            } else {
                return vec![];
            }
        } 

        let t2 = ( -b + d.sqrt() ) / ( 2.0 * a );
        let solution2 = Point3D {
            x: p1.x * ( 1.0 - t2 ) + t2 * p2.x,
            y: p1.y * ( 1.0 - t2 ) + t2 * p2.y,
            z: p1.z * ( 1.0 - t2 ) + t2 * p2.z
        };

        let sol1_vec3d = Vector3D::from_point_to_point(&ray.origin, &solution1);
        let sol2_vec3d = Vector3D::from_point_to_point(&ray.origin, &solution2);

        //println!("sol1_vec3d {:?}", sol1_vec3d);
        //println!("sol2_vec3d {:?}", sol2_vec3d);

        let solution_1_visible = ray.is_point_on_ray(&solution1);
        let solution_2_visible = ray.is_point_on_ray(&solution2);
        
        //println!("solution_1_visible {:?}", solution_1_visible);
        //println!("solution_2_visible {:?}", solution_2_visible);

        if !solution_1_visible && !solution_2_visible {
            return vec![];
        }

        if solution_1_visible && solution_2_visible {
            if sol1_vec3d.length() < sol2_vec3d.length() {
                return vec![solution1, solution2];
            } else {
                return vec![solution2, solution1];
            }
        }

        // What to do here. We're in the middle of a sphere...
        panic!("Camera is in a sphere! I don't know what to do :(");
    }
}