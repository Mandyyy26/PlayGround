fn main() {
    let x: i32 = -10;
    let y: u32 = 1000;
    let z: f32 = 1000.001;
    let a: bool = true;
    let b: char = 'a';
    let c: &str = "Hello, world!";

    println!("{}",x);
}

all variable are immutable by default.

we can make them mutable by adding mut keyword.

let mut x: i32 = 10;
x = 20;

we can also declare multiple variables at once.

let (x, y, z) = (10, 20, 30);