// Average run time of 30.011 µs for any_shape_zero_area
// Average run time of 11.633 µs for any_circle_zero_area
// Average run time of 14.261 µs for any_rectangle_zero_area

use rand::Rng;

pub trait Shape {
    fn area(&self) -> f64;
    fn description(&self) -> &str; // Used to inspect types during testing
}

#[derive(Debug, Clone)]
pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    pub fn random() -> Circle {
        Circle {
            radius: rand::random::<f64>() + 1.0,
        }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    fn description(&self) -> &str {
        "circle"
    }
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    pub fn random() -> Rectangle {
        Rectangle {
            width: rand::random::<f64>() + 1.0,
            height: rand::random::<f64>() + 1.0,
        }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn description(&self) -> &str {
        "rectangle"
    }
}

pub fn any_circle_zero_area(shapes: &Vec<Box<Circle>>) -> bool {
    shapes.iter().any(|circle| circle.area() == 0.0)
}

pub fn any_rectangle_zero_area(shapes: &Vec<Box<Rectangle>>) -> bool {
    shapes.iter().any(|rectangle| rectangle.area() == 0.0)
}

pub fn any_shape_zero_area(shapes: &Vec<Box<dyn Shape>>) -> bool {
    shapes.iter().any(|shape| shape.area() == 0.0)
}

pub fn make_circle_vec(n: usize) -> Vec<Box<Circle>> {
    let mut rng = rand::thread_rng();
    (0..n * 2).map(|_| Box::new(Circle::random()) as Box<Circle>).collect()
}

pub fn make_rectangle_vec(n: usize) -> Vec<Box<Rectangle>> {
    let mut rng = rand::thread_rng();
    (0..n * 2)
        .map(|_| Box::new(Rectangle::random()) as Box<Rectangle>)
        .collect()
}

pub fn make_mixed_vec(n: usize) -> Vec<Box<dyn Shape>> {
    let circles: Vec<Box<dyn Shape>> = (0..n).map(|_| Box::new(Circle::random()) as Box<dyn Shape>).collect();
    let rectangles: Vec<Box<dyn Shape>> = (0..n)
        .map(|_| Box::new(Rectangle::random()) as Box<dyn Shape>)
        .collect();

    let mut mixed_shapes: Vec<Box<dyn Shape>> = Vec::new();
    mixed_shapes.extend(circles);
    mixed_shapes.extend(rectangles);
    mixed_shapes
}
