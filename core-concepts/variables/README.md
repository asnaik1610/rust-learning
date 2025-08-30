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