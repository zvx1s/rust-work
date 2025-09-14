#![allow(unused)]

// Overflow doesn't panic when compiled with --release
fn main() {
    let mut x = u32::MAX;
    x += 1;

    println!("u32 max: {}, x: {}", u32::MAX, x);

    // checked_add isa function that returns None(error) on overflow
    let x = u32::checked_add(u32::MAX, 1);
    println!("checked_add: {:?}", x);
    // wrapping_add is a function that allows overflow
    let x = u32::wrapping_add(u32::MAX, 1);
    println!("checked_add: {:?}", x);
}
