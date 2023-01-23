#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!{
        "The area of the rectangle is {} square pixels.", area(&rect1)
    };

    println!("rect1:? is {:?}", rect1);
    println!("rect1:#? is {:#?}", rect1);

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area(),
    );

    let sq = Rectangle::square(3);
    println!("sq:#? is {:#?}", sq);

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
