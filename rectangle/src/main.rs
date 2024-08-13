#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle{
        width: dbg!(30 * scale),
        height: 50
    };

    println!("Rectangle is {:#?}", rect1);
    println!("Area is {}", area(&rect1));

    let rect2 = Rectangle{
        width: 25 * scale,
        height: 55
    };
    println!("Area is {}", rect1.area());
}


