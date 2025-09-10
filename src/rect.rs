#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    pub fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
    
//     let rect2 = Rectangle::square(100);
    

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }