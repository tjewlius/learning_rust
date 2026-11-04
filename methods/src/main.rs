fn main() {
    let rect1 = Rect{ width: 12, length: 18 };
    println!("Area: {}", rect1.area())
}

struct Rect {
    width: u32,
    length: u32
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.length
    }
}