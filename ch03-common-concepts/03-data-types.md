## Data Types ##

Rust is *statically-typed*, which means that all variable data types must be known at compile time.

---

### Scalar Data Types ###

Scalar data types represent a single value and come in four flavors:

* Integer
* Floating-Point
* Boolean
* Character

---

#### Integer Types ####

Where *n* is the number of bits in the integer type...
* Signed type values range from -(2ⁿ⁻¹) to 2ⁿ⁻¹ - 1
* Unsigned type values range from 0 to 2ⁿ - 1

Several integer types are available (see table below):

|Length|Similar Type|Signed|Unsigned|
|-|-|-|-|
|8-bit|byte|i8|u8|
|16-bit|short|i16|u16|
|32-bit|int|i32|u32|
|64-bit|long|i64|u64|
|128-bit|-|i128|u128|
|arch|architecture-dependent|isize|usize|
|||||

Integers support use of an underscore (a la Python) as a separator.
```rust
let a_million = 1_000_000;
```

Integer literals can be expressed as:

|Number Literals|Example|
|-|-|
|Decimal|255|
|Hexadecimal|0xff|
|Octal|0o377|
|Binary|0b1111_1111|
|Byte (u8 only)|b'\*' (* = any ASCII char)|
|||

---

#### Floating-Point Numbers ####

There are two floating-point primitives in Rust: f32 and f64.

The default is f64 (on modern PCs, the speed difference is minimal between
single- and double-precision floats).

Rust floating-point types adhere to the
[IEEE-754](https://en.wikipedia.org/wiki/IEEE_754) standard

---

#### Boolean ####

The Boolean (bool) type is one byte in size and has two possible values:
```true``` and ```false```.

```rust
fn main() {
    // implicit bool
    let t = true;

    // explicit bool
    let f: bool = false;
}
```

---

#### Character ####

The character (char) type is four bytes in size and can contain any single
Unicode character.

In Rust:

* Characters are expressed in single-quotes
* Strings are expressed in double-quotes

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ'; // Explicit type declaration
    let smiley = '☺';
    let emoji = '🙂';
    println!("{c} {z} {smiley} {emoji}");
}

// Output in pwsh: "z ℤ ☺ 🙂"
```

---

### Compound Data Types ###

---
