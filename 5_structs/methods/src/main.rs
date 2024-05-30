struct Rectangle {
    width: u32,
    heigth: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.heigth
    }

    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.heigth *= factor;
    }

    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            heigth: size,
        }
    }
}

fn main() {
    let mut r1 = Rectangle {
        width: 30,
        heigth: 50,
    };

    println!("Area is {}", r1.area());

    r1.scale(2);

    println!("Area after scale is {}", r1.area());

    let s1 = Rectangle::square(10);
    println!("Square size is {}", s1.area());
}
