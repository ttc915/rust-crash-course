#![allow(unused)]

fn main() {
    // signed integers
    let i0: i8 = -128;

    // unsigned integers
    let u0: u8 = 255;

    // floating point
    let f0: f32 = 3.11;

    // boolean
    let b0: bool = true;

    // character
    let c0: char = 'a';

    // type conversion
    let i: i32 = -1;
    let u: u32 = i as u32;
    println!("{u}");

    // min and max
    let i_max = i32::MAX;
    let u_min = u32::MIN;

    println!("i max :{i_max}");
    println!("u min :{u_min}");
} 
