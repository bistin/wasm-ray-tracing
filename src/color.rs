// from https://github.com/nwtgck/ray-tracing-iow-rust/blob/develop/src/Color.rs

use std::ops::{Add, Neg, Sub, Mul, Div};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(value R: {}, value G: {}, value B: {})", self.r, self.g, self.b)
    }
}

impl Color {
    // Length
    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }
    // Squared length
    pub fn squared_length(&self) -> f32 {
        self.r * self.r + self.g * self.g + self.b * self.b
    }
    // Unit vector
    pub fn unit_vector(&self) -> Color {
        self / self.length()
    }
    // Inner product
    pub fn dot(&self, rhs: &Color) -> f32 {
        self.r * rhs.r + self.g * rhs.g + self.b * rhs.b
    }

    pub fn to_str(&self) -> String {
    
        format!("(value r: {}, value g: {}, value b: {})", self.r, self.g, self.b)
    }
}

// why &Color
// reference https://stackoverflow.com/questions/24594374/how-can-an-operator-be-overloaded-for-different-rhs-types-and-return-values

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color{r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b}
    }
}

impl Neg for Color {
    type Output = Color;

    fn neg(self) -> Self::Output {
        Color{r: -self.r, g: -self.g, b: -self.b}
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color{r: self.r - rhs.r, g: self.g - rhs.g, b: self.b - rhs.b}
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color {r: self.r * rhs, g: self.g * rhs, b: self.b * rhs}
    }
}

impl Div<f32> for &Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Self::Output {
        Color {r: self.r / rhs, g: self.g / rhs, b: self.b / rhs}
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Self::Output {
        Color {r: self.r / rhs, g: self.g / rhs, b: self.b / rhs}
    }
}