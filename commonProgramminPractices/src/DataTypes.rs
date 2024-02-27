/* Scalar Types: representing a single value */


/* 1. Integer: a number without a decimal point

i8, i16, i32, i64, i128, isize for signed
u8, u16, u32, u64, u128, usize for unsigned
each type stores number from -(2^n-1) to (2^n-1) -1
Additionally, the isize and usize types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
 */

 /* Integer Overflow:
  Let’s say you have a variable of type u8 that can hold values between 0 and 255. If you try to change the variable to a value outside that range, such as 256, integer overflow will occur, which can result in one of two behaviors. When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs. Rust uses the term panicking when a program exits with an error; we’ll discuss panics in more depth in the “Unrecoverable Errors with panic!” section in Chapter 9.

When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error.

To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:

Wrap in all modes with the wrapping_* methods, such as wrapping_add.
Return the None value if there is overflow with the checked_* methods.
Return the value and a boolean indicating whether there was overflow with the overflowing_* methods.
Saturate at the value’s minimum or maximum values with the saturating_* methods.
 */

 /* 2. Floating point numbers: numbers with decimal points
    f32: 32 bits, f64: 64 bits
    fn main() {
        let x = 2.0; //f64
        let y: f32 = 3.0; //f32
    }
  */

  /* 3. Numeric Operations:
  Addition, Subtraction , Multiplication, Division and Remainder
   */

  /* 4. Boolean Type:
    can have true and false values only. 
    let t = true;
    let f:bool = false; //explicit type annotation
   */

  /* 5. Character Type:
  Aplhabets. 
  fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust. Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a char is in Rust
   */