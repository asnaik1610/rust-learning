fn main() {
    /* entry point -> main()
    variables are immutable by default
    "=" operator for binding
    integer types -> signed(i) & unsigned(u) */

    // signed
    let signed_number1: i8 = -10; // 8 bits
    let signed_number2: i16 = 110; // 16 bits
    let signed_number3: i32 = -1223; // 32 bits
    let signed_number4: i64 = 13334; // 64 bits
    let signed_number5: i128 = -144455566; // 128 bits
    let signed_number6: isize = 5436575473646; // depends on the computer's architecture

    // unsigned
    let unsigned_number1: u8 = 10;
    let unsigned_number2: u16 = 110;
    let unsigned_number3: u32 = 1223;
    let unsigned_number4: u64 = 13334;
    let unsigned_number5: u128 = 144455566;
    let unsigned_number6: usize = 5436575473646;

    // inferred
    let my_number_default = 50; // it automatically chooses i32 for integers

    // to make a variable mutable -> mut
    let mut no_of_apples = 4;

    // different ways to declare an immutable variable
    let varname1: u8 = 90;
    let varname2 = 13; // inferred
    let varname3: i8;
    // let varname4;
}
