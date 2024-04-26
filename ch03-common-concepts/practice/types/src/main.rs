fn main() {
    // SCALAR VARIABLES

    // Integers
    // Length	Signed	Unsigned
    // ------------------------
    //  8-bit   i8      u8
    //  16-bit  i16     u16
    //  32-bit  i32     u32
    //  64-bit  i64     u64
    //  128-bit i128    u128
    //  arch    isize   usize
    let int_x = 2;         // i32
    let int_y: usize = 3;  // usize
    println!("Integers: {int_x} {int_y}");

    // Integer Overflow
    let mut o: u8 = 0;
    // o -= 1; // In debug, this panics - In release, this returns 255 (messy)

    // Literal Syntax
    // Number literals	Example
    // ------------------------
    // Decimal          98_222
    // Hex              0xff
    // Octal            0o77
    // Binary           0b1111_0000
    // Byte (u8 only)   b'A'
    let dec: u32 = 98_222;
    let hex: u16 = 0xff;
    let oct: u16 = 0o77;
    let bin: u16 = 0b1111_0000;
    let byt: u8 = b'A';
    println!("Literals: {dec} {hex} {oct} {bin} {byt}");

    // Floating Point
    // Standard: https://en.wikipedia.org/wiki/IEEE_754
    let flt_x = 2.0;       // f64 (double-precision)
    let flt_y: f32 = 3.0;  // f32 (single-precision)
    let flt_pi = 3.14159;  // f64
    println!("Floating Point: {flt_x} {flt_y} {flt_pi}");

    // Boolean
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("Boolean: {t} {f}");

    // Characters
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("Characters: {c} {z} {heart_eyed_cat}")
}
