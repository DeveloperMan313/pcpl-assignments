trait Shape {
    fn area(&self) -> f32;
}

trait IPrint: ToString {
    fn print(&self) {
        println!("{}", self.to_string());
    }
}

struct Rectangle {
    width: f32,
    height: f32,
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        return self.width * self.height;
    }
}

impl ToString for Rectangle {
    fn to_string(&self) -> String {
        return format!(
            "Rectangle with width = {}, height = {} and area = {}",
            self.width,
            self.height,
            self.area()
        );
    }
}

impl IPrint for Rectangle {}

struct Square {
    side: f32,
}

impl Shape for Square {
    fn area(&self) -> f32 {
        return self.side * self.side;
    }
}

impl ToString for Square {
    fn to_string(&self) -> String {
        return format!(
            "Square with side = {} and area = {}",
            self.side,
            self.area()
        );
    }
}

impl IPrint for Square {}

struct Circle {
    radius: f32,
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        return std::f32::consts::PI * self.radius * self.radius;
    }
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        return format!(
            "Circle with radius = {} and area = {}",
            self.radius,
            self.area()
        );
    }
}

impl IPrint for Circle {}

fn main() {
    let rectangle = Rectangle {
        width: 5.0,
        height: 10.0,
    };
    let square = Square { side: 7.5 };
    let circle = Circle { radius: 12.0 };

    rectangle.print();
    square.print();
    circle.print();
}
