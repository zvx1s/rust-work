#![allow(unused)]

// Constants(can be outside main function and lives inside compiled code)
const NUM: u32 = 1;

fn main() {
    // Variables
    // - Immutable by default
    // - Use mut to make it mutable
    let mut x = 1;
    x += 1;

    // Type inference
    let y = -1;
    let z = -1;

    //Shadowing
    let x = 1;
    let x = 2;
    let x: bool = true;

    //Type placeholder
    let x: _ = 1234;

    let x = 1;
    println!("x is {}", x);
    //Inline
    println!("x is {x}");
    // Positional arguments
    let z = x + x;
    println!("{0} + {0} = {1}", x, x + x);
    // Debug
    println!("DEBUG: x {:?}", x);
    println!("DEBUG: x {:#?}", x);
}
