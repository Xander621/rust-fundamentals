# Ultimate Rust Crash Course

Author: Nathan Stocks

## Sources

Github: CleanCut/utlimate_rust_crash_course

### Cargo Package Manager

* Use cargo to search for, install, and manage packages you want to use in your program.
* Cargo is also the build system. No more make files!
* Cargo is also the test runner, and the documentation generator, etc...

### First Program

```rust
cargo new hello
```

* Creates directory named hello
  * Creates Cargo.toml
    * uses .toml format -> Tom's obvious minimal language
  * main.rs

```rust
cargo run
```

builds project with debug symbols under target/debug/

```rust
cargo run --release
```

builds project w/o debug symbols under target/release/

### Scalar Types

* u8 - u128 -> unsigned integers
* i8 - i128 -> signed integers
* usize - is the size of the platform's pointer type and can represent evry memory address
  * type you will ususally use to index into an array or vector.
* isize - integer type
  * max isize value is the upper bound of the object and array size. This ensures that isize can be used to calculate differences between pointers and be able to address every byte w/in a value like a struct.
* If you don't annotate an integer literal then it defaults to i32 because it's generally the fastest integer even on 64-bit architectures.
* Integer literals
  * Decimal is what you'd expect -> 1000000
  * Hex -> 0x
  * Octal -> 0o
  * Binary -> 0b
  * Byte (u8 only) b'A' single-quotes enclosing a UTF-8 character in the ASCII range.
    * pretty rare to use... most people just use a decimal integer between 0 and 255.
    * the terms u8 and byte are used interchangeably.
  * Number representations that take more than one digit can have any number of ignored underscores inside them or at the end of them.
    * Example: 1_000_000
* Floating Point types
  * f32 has 32 bits of precision
  * f64 has 64 bits of precision
    * is the default because it has more precision, but it can be really slow on less that 64-bit architectures, so be careful with that.
  * Floating point literals follow the IEEE-754 standard but basically look like this:
    * 3.14159 -> valid
    * .1 -> not valid - need a value before the decimal
* Usually when you want a specific type, you will annotate a variable declaration and the type of the litera

    ```rust
    let x: u16 = 5;
    let y: f32 = 3.14;
    ```

* it is also completely fine to suffix the literal with the type you want.
  * This is especially useful if you want to pass a literal to a generic function that could accept multiple numeric types!
  * This is one situation where underscores can really improve readability.

```rust
let x = 5u16;
let y = 3.14f32;
    -- or --
let x = 5_u16;
let y = 3.14_f32;
```

* Boolean Type - bool
  * The two boolean literals are true and false, all lowercase.
  * Booleans are not integers, so don't try to use arithmetic on them, it wont work, unless you cast them to some integer type like this:

```rust
true as u8
false as u8
```

* The Character Type - char
  * It represents a single unicode scalar value which could be anything from a character of our alphabet to a character of another's alphabet to an ideograph or a diacritic or an emoji or a non-printable Unicode control character that could represent anything from a sound to an action.
  * A character is always 4 bytes which effectively makes an array of characters a UCS-4 or UTF-32 string.
  * Character literals are specified using single quotes, and most importantly, characters are fairly useless.

```rust
let my_letter = 'a';
```

* Strings are UTF-8 and characters are not, so strings do not use characters internally.
* Source files are also UTF08, so chances are when you want to deal with a single character it's going to be a UTF-8 string not a character literal.

### Compound Types

Compound types gather multipel values of other types into one type.

* Tuple - Store multiple values of ***any*** type.

    ```rust
    let info = (1, 3.3, 999);
    let info: (u8, f64, i32) = (1, 3.3, 999);
    ```

  * Two ways to access members of a tuple
        1. use dot syntax, a.k.a. a field access expression
        2. the second way is to access members of a tuple all at once

    ```rust
    // field access expression - 0 indexed
    let info = (1, 3.3, 999);
    let jets = info.0;
    let fuel = info.1;
    let ammo = info.2;

    // 2nd way uses a pattern to destructure and access all the elements of a tuple
    let (jets, fuel, ammo) = info;
    ```

  * Be aware that tuples currently have a maximum ***arity*** of twelve above which you can technically still use the tuple, but w/ limited functionality.
    * Arity means how many items are in the tuple.

```rust
// Tuple with an arity of 4
(u8, u8, i32, u64)
```

* Array - Store multiple values of the ***same*** type

    ```rust
    let buf = [1, 2, 3];
    
    // or with a value wiht how many you want, seperated w/ semicolon
    let buf = [0; 3];

    // w/ type annotation
    let buf: [u8; 3] = [1, 2, 3];
    ```

  * Index values of an array w/ square brackets

    ```rust
    buf[0]
    ```

  * Arrays are limited to a size of 32, above which they lose most of their functionality.
    * Arrays live on the stack by default and are fixed size, so you will ususally use ***vectors*** or slices of vectors instead of arrays.
