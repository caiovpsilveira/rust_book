struct Rectangle {
    width: u32,
    heigth: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.heigth
    }
}

struct Circle {
    radius: u32,
}

impl Circle {
    fn area(&self) -> u32 {
        // Approx PI = 3
        self.radius * self.radius * 3
    }
}

struct Triangle {
    base: u32,
    height: u32,
}

impl Triangle {
    fn area(&self) -> u32 {
        self.base * self.height / 2
    }
}

enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Triangle(Triangle),
}

fn main() {
    println!("Hello, world!");

    let r = Rectangle {
        width: 10,
        heigth: 10,
    };

    let c = Circle { radius: 50 };

    let t = Triangle {
        base: 100,
        height: 100,
    };

    let arr: [Shape; 3] = [Shape::Rectangle(r), Shape::Circle(c), Shape::Triangle(t)];

    for shape in arr {
        match shape {
            Shape::Rectangle(shape) => println!("Found a rectangle with area {}!", shape.area()),
            Shape::Circle(shape) => println!("Found a circle with area {}!", shape.area()),
            Shape::Triangle(shape) => println!("Found a triangle with area {}!", shape.area()),
        }
    }

    // println!("{}", r.area()); // does not compile, "r" was moved
}
