#![allow(unused)]

pub fn mul(x: u32, y: u32) -> u32 {
    x * y
}

pub fn div(x: u32, y: u32) -> u32 {
    x / y
}

fn add_with_return(x: u32, y: u32) -> u32 {
    return x + y;
}

fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn print(s: String) {
    println!("{s}{s}{s}{s}{s}")
}

fn main() {
    let x = 1;
    let y = 2;
    let z = add(x, y);
    println!("{x} + {y} = {z}");
    let a = mul(x, y);
    println!("{a}");
    print("🐸".to_string());
}
