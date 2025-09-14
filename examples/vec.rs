#![allow(unused)]

pub fn init(x: u32, y: u32, z: u32) -> Vec<u32> {
    vec![x, y, z]
}



fn main() {
    // Create an empty Vec and push values
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v: {:?}", v);

    // Different ways to create vectors
    let v: Vec<i8> = vec![1, 2, 3];
    println!("v: {:?}", v);

    let v = vec![1u8, 2, 3];
    println!("v: {:?}", v);

    let v: Vec<i8> = vec![0i8; 100];
    println!("v: {:?}", v);

    // Get element by index
    let v: Vec<i8> = vec![1, 2, 3];
    println!("v[1]: {}", v[1]);

    // Safe access with .get() (returns Option<&i8>)
    println!("v.get(1): {:?}", v.get(1));
    println!("v.get(1000): {:?}", v.get(1000)); // invalid index -> None

    // Update element
    let mut v: Vec<i8> = vec![1, 2, 3];
    v[0] = 99;
    println!("updated v: {:?}", v);

    // Pop elements (remove last element)
    let mut v: Vec<i8> = vec![1, 2, 3];
    let x: Option<i8> = v.pop();
    println!("pop: {:?}", x); // Some(3)
    let x: Option<i8> = v.pop();
    println!("pop: {:?}", x); // Some(2)
    let x: Option<i8> = v.pop();
    println!("pop: {:?}", x); // Some(1)
    let x: Option<i8> = v.pop();
    println!("pop: {:?}", x); // None

    // Slice a vector
    let v = vec![1, 2, 3, 4, 5];
    let s = &v[1..4];
    println!("slice: {:?}", s); // [2, 3, 4]
}
