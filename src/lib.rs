pub mod two_d {
    pub struct Rectangle {
        pub length: f64,
        pub width: f64
    }
    pub struct Square {
        pub a: f64
    }
    pub struct Circle {
        pub radius: f64
    }
    pub struct Triangle {
        pub a: f64,
        pub b: f64,
        pub c: f64
    }
}

pub mod three_d {
    pub struct Cube {
        pub a: f64
    }
    pub struct Cuboid {
        pub length: f64,
        pub width: f64,
        pub height: f64
    }
    pub struct Sphere {
        pub radius: f64
    }
}

use crate::two_d::Triangle;
use crate::two_d::Square;
use crate::three_d::Sphere;
use crate::two_d::Rectangle;
use crate::three_d::Cube;
use crate::three_d::Cuboid;
use crate::two_d::Circle;
use std::f64::consts::PI;


impl Rectangle {
    pub fn new(length: f64, width: f64) -> Result<Self, String> {
        if length >= 0.0 && width >= 0.0 {
            Ok(Self {length, width})
        } else {
            return Err("Rectangle parameters are invalid. Negative numbers.".to_string());
        }
    }
    pub fn area(&self) -> f64 {
        return self.length * self.width
    }
    pub fn perimeter(&self) -> f64 {
        return (self.length + self.width) * 2.0
    }
    pub fn diagonal(&self) -> f64 {
        return (self.length.powi(2) + self.width.powi(2)).sqrt()
    }
}

impl Cuboid {
    pub fn new(length: f64, width: f64, height: f64) -> Result<Self, String> {
        if length >= 0.0 && width >= 0.0 && height >= 0.0 {
            return Ok(Self {length, width, height})
        } else {
            return Err("Cuboid parameters are invalid. Negative numbers".to_string());
        }
    }
    pub fn surface_area(&self) -> f64 {
        return 2.0 * (self.length * self.height + self.width * self.height + self.width * self.length)
    }
    pub fn volume(&self) -> f64 {
        return self.length * self.height * self.width
    }
    pub fn diagonal(&self) -> f64 {
        return (self.width.powi(2) + self.length.powi(2) + self.height.powi(2)).sqrt()
    }
}

impl Circle {
    pub fn new(radius: f64) -> Result<Self, String> {
        if radius >= 0.0 {
            Ok(Self {radius})
        } else {
            return Err("Circle parameter are invalid. Negative number".to_string());
        }
    }
    pub fn area(&self) -> f64 {
        return self.radius.powi(2) * PI
    }
    pub fn perimeter(&self) -> f64 {
        return self.radius * 2.0 * PI
    }
}

impl Cube {
    pub fn new(a: f64) -> Result<Self, String> {
        if a >= 0.0 {
            Ok(Self {a})
        } else {
            return Err("Cube parameter are invalid. Negative number".to_string());
        }
    }
    pub fn surface_area(&self) -> f64 {
        return self.a.powi(2) * 6.0
    }
    pub fn volume(&self) -> f64 {
        return self.a.powi(3)
    }
    pub fn diagonal(&self) -> f64 {
        return (self.a.powi(2) * 3.0).sqrt();
    }
}

impl Square {
    pub fn new(a: f64) -> Result<Self, String> {
        if a >= 0.0 {
            Ok(Self {a})
        } else {
            return Err("Square parameter are invalid. Negative number".to_string());
        }
    }
    pub fn area(&self) -> f64 {
        return self.a.powi(2)
    }
    pub fn perimeter(&self) -> f64 {
        return self.a * 4.0
    }
    pub fn diagonal(&self) -> f64 {
        return (self.a.powi(2) * 2.0).sqrt();
    }
}

impl Sphere {
    pub fn new(radius: f64) -> Result<Self, String> {
        if radius >= 0.0 {
            Ok(Self {radius})
        } else {
            return Err("Sphere parameter are invalid. Negative number".to_string());
        }
    }
    pub fn volume(&self) -> f64 {
        return (4.0 / 3.0) * PI * self.radius.powi(3)
    }
    pub fn surface_area(&self) -> f64 {
        return 4.0 * PI * self.radius.powi(2)
    }
}

impl Triangle {
    pub fn new(a: f64, b: f64, c: f64) -> Result<Self, String> {
        if a >= 0.0 && b >= 0.0 && c >= 0.0 {
            if a + b == c && c + b == a && c + a  == b {
                Ok(Self {a, b, c})
            } else {
                return Err("Invalid triangle. Sum of 2 sides can't make 3rd side".to_string());
            }
        } else {
            return Err("Triangle parameters are invalid. Negative numbers".to_string());
        }
    }
    pub fn area(&self) -> f64 {
        let p: f64 = (self.a + self.b + self.c) / 2.0;
        return (p * (p - self.a) * (p - self.b) * (p - self.c)).sqrt();
    }
    pub fn perimeter(&self) -> f64 {
        return self.a + self.b + self.c
    }
}

