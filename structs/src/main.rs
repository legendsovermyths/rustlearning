#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.height*self.width
    }
    fn contains(&self, other:&Rectangle)->bool{
        if other.width<self.width && other.height < self.height{
            return true;
        }
        false
    }
    fn square(side:u32)->Rectangle{
        Rectangle{
            width:side,
            height:side,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle{
        width: 25,
        height: 45,
    };
    let sq1 = Rectangle::square(32);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Rectangle 2 can fit in Reactangle 1: {}",rect1.contains(&rect2));
    println!("{:?}",sq1);
}

