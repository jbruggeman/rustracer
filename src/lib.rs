mod ray_trader {
    struct Point {
        x: f64,
        y: f64,
        z: f64,
    }

    struct Color {
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    }

    struct Viewport {
        width: u32,
        height: u32,
    }

    struct Camera {
        position: Point,
        target: Point,
        fov: f64
    }

    enum ObjectType {
        Sphere,
        Light
    }

    struct Sphere {
        color: Color,
        position: Point,
        radius: f64
    }

    struct Light {
        position: Point
    }

    struct Objects {
        spheres: Vec<Sphere>,
        lights: Vec<Light>,
    }
}