#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
	fn area(&self)->u32{
		self.width*self.height
	}

	fn can_hold(&self, other: &Rectangle)-> bool {
		self.width > other.width && self.height > other.height
	}

    fn square(size: u32) -> Self{
		Self {
			width: size,
			height: size,
		}
	}
}
fn main() {
    let scale = 2;
    let r1 = Rectangle{
        width: 30,
        height: dbg!(50 * scale), 
    };
    
    let r2 =  Rectangle{
        width: 20,
        height: dbg!(20 * scale), 
    };
    dbg!(r1.area());
    dbg!(r1.can_hold(&r2));

    let s = Rectangle::square(30);
    dbg!(&r1);
    dbg!(&s);
}


