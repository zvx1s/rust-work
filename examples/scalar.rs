#![allow(unused)]

pub fn eq(a: char, b: char) -> bool {
    a == b
}

pub fn add(a: f32, b: f32, c: f32) -> f32 {
    a + b + c
}

pub fn cast(x: u8, y: i8, z: f32) -> f32 {
    x as f32 + y as f32 + z
}

// Scalar - data types that represent a single value
fn main() {
    // Signed integers(Can be both negative and positive)
    // Unsigned integers
    // Boolean
    // Floating point
    // Characters(single quote)

    //Type conversion:
    let i: i32 = -1;
    let u = i as u32;
    println!("{i} as u32 = {u}");

    // Min and max
    let i_max = i32::MAX;
    let u_min = u32::MIN;
    println!("i max: {i_max}");
    println!("u min: {u_min}");
}
