#![allow(unused)]

#[derive(Debug, PartialEq)]
enum Command {
    Play,
    Stop,
    Skip(u32),
    Back(u32),
    Resize { width: u32, height: u32 },
}

pub enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
}

// Enum
fn main() {
    let cmd: Command = Command::Play;
    let cmd: Command = Command::Skip(10);
    let cmd: Command = Command::Resize {
        width: 100,
        height: 50,
    };

    // Debug
    println!("{:?}", cmd);
    let cmd0: Command = Command::Play;
    let cmd1: Command = Command::Skip(10);
    println!("{}", cmd0 == cmd1);

    // Option
    let x: Option<i32> = Some(1); // x contains an integer
    let x: Option<i32> = None; // x contains nothing
    // Result
    let x: Result<i32, String> = Ok(100);
    // ✅ Success: got a number

    let x: Result<i32, String> = Err("Failed to parse string into number".to_string());
    // ❌ Failure: include an error message
}
