use serde::Deserialize;

#[derive(Deserialize, Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn as_float_color(&self) -> FloatColor {
        FloatColor {
            r: (self.r as f64) / 255.0,
            g: (self.g as f64) / 255.0,
            b: (self.b as f64) / 255.0
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct FloatColor {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl FloatColor {
    pub fn as_color(&self) -> Color {
        Color {
            r: (self.r * 255.0) as u8,
            g: (self.g * 255.0) as u8,
            b: (self.b * 255.0) as u8
        }
    }

    pub fn black() -> FloatColor {
        FloatColor {
            r: 0.0,
            g: 0.0,
            b: 0.0
        }
    }

    pub fn bound(&self) -> FloatColor {
        let mut new = *self;

        if new.r < 0.0 {
            new.r = 0.0;
        } else if new.r > 1.0 {
            new.r = 1.0;
        }
        if new.g < 0.0 {
            new.g = 0.0;
        } else if new.g > 1.0 {
            new.g = 1.0;
        }
        if new.b < 0.0 {
            new.b = 0.0;
        } else if new.b > 1.0 {
            new.b = 1.0;
        }

        new
    }
}

impl std::ops::Mul<f64> for FloatColor {
    type Output = FloatColor;

    fn mul(self, scalar: f64) -> FloatColor {
        FloatColor {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }
}

impl std::ops::Mul<FloatColor> for FloatColor {
    type Output = FloatColor;

    fn mul(self, other: FloatColor) -> FloatColor {
        FloatColor {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl std::ops::Add<FloatColor> for FloatColor {
    type Output = FloatColor;

    fn add(self, other: FloatColor) -> FloatColor {
        FloatColor {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl std::ops::AddAssign for FloatColor {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}
