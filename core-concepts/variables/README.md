- The type system is the thread that **binds** an application
- Rust → **strongly typed language**
- Variables are **statically typed**
- Even with type inference, a variable is assigned an inferred type, which will **never change**
- Rust is more flexible at inferring the correct type than other languages, including indirect inferences
- **Immutability** is The Default in Rust
- Memory is where data resides
- Variables are named memory storage, and they remove the need to reference data using memory addresses 
- A type describes the **memory layout of a value**
- Variable Binding is an important term in Rust
- `let` statement → creates binding between a variable name and a memory location
- Rust supports **flexible binding**

---

### 📜The following are the rules and naming conventions for variable names:
1. They are case-sensitive.
2. They consist of alphanumeric characters and underscore.
3. They cannot start with a number.
4. The naming convention is snake case.

---

- Variables must be initialized before using them in some manner

---

### Primitive Data Types
- basic types & building blocks of Rust
- are intrinsic in the sense that:
   - The compiler knows their size, layout, and behavior.
   - Operations on them (like +, -, *) are compiled directly to machine instructions.
   - They don’t require the standard library to exist.
   - That’s why you can use them in **#![no_std]** environments (like embedded systems).
- implemented by Rust compiler
- functions & attributes of primitives are implemented in Rust libraries 
#### Types of Primitives
##### 1. Scalar
    - signed integer
    - unsigned integer
    - float
    - bool
    - reference 
##### 2. Non-Scalar
    - array
    - tuple
    - slice
    - String
    - str  
##### 3. Other 
    - (): unit type
    - fn: function pointer types
    - raw pointer
  
----
#### Integer Types
- Except for `isize` and `usize`, integer types are fixed sized, where the suffix of the type name defines the bit size.
- The following are the signed integer types:
  - i8
  - i16
  - i32
  - i64
  - i128
  - isize
- The following are the unsigned integer types:
  - u8
  - u16
  - u32
  - u64
  - u128
  - usize
- Type sizes in Rust are more discrete than in some other languages. This enables developers to craft applications that are more efficient and specific to their requirements. 
  ##### Range of Values for Signed Integer Types

| Type  | Size    | Range                                                                                                           |
| ----- | ------- | --------------------------------------------------------------------------------------------------------------- |
| i8    | 8-bit   | –128 to 127                                                                                                     |
| i16   | 16-bit  | –32,768 to 32,767                                                                                               |
| i32   | 32-bit  | –2,147,483,648 to 2,147,483,647                                                                                 |
| i64   | 64-bit  | –9,223,372,036,854,775,808 to 9,223,372,036,854,775,807                                                         |
| i128  | 128-bit | –170,141,183,460,469,231,731,687,303,715,884,105,728 to <br>170,141,183,460,469,231,731,687,303,715,884,105,727 |
| isize | Pointer | Architecture dependent                                                                                          |

##### Range of Values for Unsigned Integer Types

| Type  | Size    | Range                                                    |
| ----- | ------- | -------------------------------------------------------- |
| u8    | 8-bit   | 0 to 255                                                 |
| u16   | 16-bit  | 0 to 65,535                                              |
| u32   | 32-bit  | 0 to 4,294,967,295                                       |
| u64   | 64-bit  | 0 to 18,446,744,073,709,551,615                          |
| u128  | 128-bit | 0 to 340,282,366,920,938,463,463,374,607,431,768,211,455 |
| usize | Pointer | Architecture dependent                                   |


- The size of the `isize` and `usize` types depends on the **operating environment**. **It is the size of a pointer**.
- You can get the size of any type with the `size_of` method function.
- Overflow
  - An integer overflow or underflow occurs when the minimum or maximum value of an integer type is exceeded.
  - When an overflow occurs, the result is dependent upon whether the binary is a debug or release build. 
  - If it’s a debug build, a panic will occur when there is an overflow. 
  - Panics are exceptional events that can interrupt an application if unhandled. However, a panic does not occur when an overflow occurs within a release build. The overflow rotates the number from the maximum value to the minimum, or vice versa. 
  - To get consistent results of an overflow → use `overflowing_add` function which return a tuple with two values → result and overflow status. 
    - Similar functions → `overflowing_sub`, `overflowing_mul`, `overflowing_div`, and `overflowing_pow`
 ---

#### Floating Point Types
- Rust has single and double precision primitive types (IEEE 754).
- Each type consists of a sign, exponent, and mantissa component.
- `f32` → single-precision (32 bits)
- `f64` → double-precision (64 bits)
- For type inference → default is `f64`
- floats are **always signed**.
- Neither of them are ideal for fixed-point numbers.
- The Decimal type, found in the `rust_decimal` crate, is a great type for fixed-point floating point numbers.
- You can create a Decimal number with `from_str` contructor or `dec!` macro.
- Floating point constants are implemented as `f64` primitives in the `std::f64::consts` module.
- Rust supports 32-bit and 64-bit versions of infinity and negative infinity. `INFINITY` and `NEG_INFINITY` are const values found in either the `std::f32` or `std::f64` module.
- NaN represents a numerically undefined or unknown result → `f64::NAN`.