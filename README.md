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

### Control Flow

#### if statement

The "if" statement in rust does not require parentheses around the condition, everything between the if and opening curly brace is the condition. The condition **must** evalutat to a boolean. Rust does not like type coercion

```rust
if num == 5 {
  msg = "five";
} else if num == 4 {
  msg = "four"
} else {
  msg = "other";
}
```

"if" is an expression, not a statement. Statements don't return values, expressions do, meaning that we can change this code to this:

```rust
// msg is assigned the value of the "if" expression
// Five things to note:
// 1. there are no semicolons after the branch values to make it so that the values get returned from the blocks as tail expressions.
// 2. you can't use return for this purpose, even if you wanted to, because return only applies to blocks that are function bodies, 
//    so return would return out of the current function.
// 3. All the blocks return the same type 
// 4. there is a semicolon at the end of the "if" expression.
//    NOTE: if you don't use the value of an "if" expression then Rust will let you cheat and leave off the semicolon, but if you do use 
//          the value of an if expression in a statement, then you need to put a semicolon after the closing brace before starting any other
//          statements in the block.
// 5. Braces are not optional in Rust.
msg = if num == 5 {
  "five"
} else if num == 4 {
  "four"
} else {
  "other
};
```

Can't use turnary expressions like c

```rust
num = a ? b : c // can't use, but
num = if a {b} else {c};

mum = a ? x ? y : z : c;  // bad
num = if a {
  if x {y} else {z}
} else {c};
```

#### String

There are at least six types of strings in teh Rust standard library. But we mostly care about two of them that overlap each other.

1. *str* - the string slice and it will almost always be use as a borrowed string slice *&str*
   1. A literal string is alwasy a borrowed string slice

   ```rust
   let msg = "hello world";
   ```

2. **S**tring

   ```rust
   let msg = "hello world".to_string(); // calling the to_string() method on a borrowed string slice
   // or
   let msg = String::from("hello world"); // passing borrowed string slice to String::from()
   ```

* The data in a borrowed string slice cannot be modified while the data in a String can be modified.
* A borrowed string slice is internally made up of a pointer to some bytes and a length.
  * ptr -> hello world
  * len -> 11
* A String is made up of a pointer to some bytes, a length and a capacity that may be higher than what is currently being used.
  * ptr -> hello world
  * len -> 11
  * capacity -> 32
* Both string types are valid UTF-8 by definition, compiler enforcement and runtime checks
* Strings cannot be indexed by character position i.e. my_string[3]

### Ownership

Ownership is what makes all those informative compiler error messages possible and necessary.

There are three rules to ownership

1. Each value has an owner. There is no value in memory, no data that doesn't have a variable that owns it.
2. There is only one owner of a value. 
   1. No variables may share ownership. 
   2. Other variables may borrow the values.
3. Value gets dropped if its owner goes out of scope.

#### Ownership in action

```rust
let s1 - String::from("abc");
let s2 = s1; // at this point the value for S1 is moved to S2 because only one variable can own the value
println!("{}", s1); // Error!

// instead
let s2 = s1.clone(); // akin to a deep copy
```

### References & Borrowing

```rust
let s1 = String::from("abc");

do_stuff(&s1); // borrows a reference to the value of s1. i.e. the reference, not the values, gets moved into the function.

fn do_stuff(s: &String) { // reference to a string
  // do stuff
}
```

By default references are always immutable, to make them mutable

```rust
let mut s1 = String::from("abc");
do_stuff(&mut s1);

fn do_stuff(S: &mut String) {

  // the dot operator for a method or a field auto-dereferences down to the actual value. 
  // You don't have to worry whether sommething is a value is a reference or even a reference to a reference.
  s.insert_str(0, "hi, ");  

  // To manually dereference s it would look like this, similar to C.
  (*s).insert_str(0, "hi, ");

  // With most other operators, like the assignment operator, you'll need to manually dereference your reference if you want 
  // to read from or write to the actual value. Here I'm dereferencing s so I can replace the entire value.
  *s = String::from("Replacement");
}
```

Summary

```rust
// immutable reference to x
&x 

// mutable reference to x
&mut x 

// type of value
i32

// type of immutable reference 
&i32

// type of mutable reference
&mut i32

// variable x is a mutable reference to a value, so dereferncing x gives you mutable access to the value
x: &mut i32
*x  // a mutable i32

// variable x is immutable, dereferencing x gives you immutable access to the value
x: &i32
*x  // an immutable i32
```

References are implemented via pointers, so naturally Rust has a special rule to keep us safe. At any given time
you can have either ***exactly one*** mutable reference or any number of immutable references.

### Structs

```rust
struct RedFox { // Capital camel case then curly braces
  enemy: bool,
  life: u8,
}

// instantiate a struct
let fox = RedFox {
  enemy: true,
  life: 70,
}

// implement an associated function to use as a constructor to create a struct... 
impl RedFox {
  fn new() -> Self {
    Self {
      enemy: true,
      life: 70,
    }
  }
}

let fox = RedFox::new();
```

### Triats

Triats are similar to interfaces in other languages. Rust takes the composition over inheritance approach.
Using our RedFox struct, let's make a trait called Noisy. Traits define required behavior.

```rust
struct RedFox {
  enemy: bool,
  life: u32,
}

trait Noisy {
  fn get_noise(&self) -> &str;
}

impl Noisy for RedFox {
  fn get_noise(&self) -> &str { "Meow?"};
}

// generic function that implments the Noisy triat
fn print_noise<T: Noisy>(item: T) {
  println!("{}", item.get_noise());
}

```