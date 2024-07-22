use std::io;
use std::io::Write;

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn binary(x: i32) {
    if x == 0 {
        return;
    }
    binary(x / 2);
    print!("{}", x % 2);
    io::stdout().flush().unwrap();
}

fn main() {
    let y: (i32, i32) = (2, 3);

    let (a,b) = y;
    binary(a);
    println!("{a}, {b}");
}
