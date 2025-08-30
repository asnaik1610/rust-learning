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
}
