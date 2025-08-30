use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::f64::consts;
use std::mem;

fn main() {
    /* entry point -> main()
    variables are immutable by default
    '=' operator for binding
    integer types -> signed(i) & unsigned(u)
    why '_' ? -> Rust way of saying -> "This value exists, but I’m intentionally ignoring it" */

    // signed
    let _signed_number1: i8 = -10; // 8 bits
    let _signed_number2: i16 = 110; // 16 bits
    let _signed_number3: i32 = -1223; // 32 bits
    let _signed_number4: i64 = 13334; // 64 bits
    let _signed_number5: i128 = -144455566; // 128 bits
    let _signed_number6: isize = 5436575473646; // depends on the computer's architecture

    // unsigned
    let _unsigned_number1: u8 = 10;
    let _unsigned_number2: u16 = 110;
    let _unsigned_number3: u32 = 1223;
    let _unsigned_number4: u64 = 13334;
    let _unsigned_number5: u128 = 144455566;
    let _unsigned_number6: usize = 5436575473646;

    // inferred
    let _my_number_default = 50; // it automatically chooses i32 for integers

    // to make a variable mutable -> mut
    let mut _no_of_apples = 4;

    // different ways to declare an immutable variable
    let _varname1: u8 = 90;
    let _varname2 = 13; // inferred
    let _varname3: i8;
    let _varname4;
    _varname4 = 89;

    // MAX of int
    let max_signed_int = i32::MAX;
    let max_unsgined_int = u32::MAX;

    // println! is a macro that prints a string to the screen
    println!("{} {}", max_signed_int, max_unsgined_int);

    let size = mem::size_of::<isize>();
    println!("The size of isize on my OS is {} bits", size * 8);

    // notations -> base 10 is the default base for integer values
    println!("{}", 10); // decimal
    println!("{:04b}", 0b10); // binary
    println!("{}", 0o42); // octal
    println!("{}", 0xA); // hexadecimal

    // float types
    // calculating the volume of sphere
    let radius_sphere: f64 = 7.0;
    let volume_sphere = (4.0 / 3.0) * consts::PI * radius_sphere.powi(3); // use powi for int exp on floats
    println!("The volume of the sphere: {:.3}", volume_sphere);

    // creating decimal
    let mut number1 = Decimal::from_str("-1.23656").unwrap();
    let mut _number2 = dec!(-1.23656); // alternative

    // round up value to 2 decimal places
    number1 = number1.round_dp(2);
    println!("The rounded number: {}", number1);
}
