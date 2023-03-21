use std::{f64::consts, fmt::Debug};

// Define a trait that can get the area of a shape
pub trait Area {
    fn area(&self) -> f64;
}

#[derive(Debug)]
pub struct Circle {
    pub radius: f64,
}

#[derive(Debug)]
pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

#[derive(Debug)]
pub struct Square {
    pub side: f64,
}

// Implement the Area trait for each struct
impl Area for Circle {
    fn area(&self) -> f64 {
        consts::PI * self.radius * self.radius
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// print the shapes' area, Taking a generic type that implements the Area trait.
pub fn print_area<T: Area + std::fmt::Debug>(shape: &T) {
    println!("{:?} area is: {}", shape, shape.area());
}
