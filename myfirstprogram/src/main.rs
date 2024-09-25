use myfirstprogram::greet;

fn main() {
    greet();
    let _bunnies = 4;
    let _rabbits: i32 = 2;
    let (dogs, _cats) = (8, 50); // tuple pattern to initialize multiple variables at once

    println!("doges = {}", dogs);
 
    // shadowing example 
    let x = 5;
    {
        let x = 99;
        println!("{}", x); // Prints "99" because the x is local to this scope, the x outside this scope is shadowed by this variable x
    }
    println!("{}", x);

    // example using for loop and iterrator
    let array = [(1, 2), (3, 4), (5, 6)];
    for (x, y) in array.iter() {
        println!("x = {}, y = {}", x, y);
    }

    // example using while loop
    let mut counter = 0;
    while counter < 10 {
        println!("counter = {}", counter);
        counter += 1;
    }

    // example using loop
    let mut counter = 0;
    loop {
        println!("counter = {}", counter);
        counter += 1;
        if counter == 10 {
            break;
        }
    }

    // example using match
    let number = 5;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        _ => println!("Something else"),
    }

    // example using if let
    let number = Some(7);
    if let Some(7) = number {
        println!("It's seven");
    } else {
        println!("It's something else");
    }

    // example using if let with else if
    let number = Some(7);
    if let Some(7) = number {
        println!("It's seven");
    } else if let Some(8) = number {
        println!("It's eight");
    } else {
        println!("It's something else");
    }

    // example using if let with else if and else
    let number = Some(8);
    if let Some(7) = number {
        println!("It's seven");
    } else if let Some(8) = number {
        println!("It's eight");
    } else {
        println!("It's something else");
    }

    // example using if let with else if and else with variable binding
    let number = Some(8);
    if let Some(7) = number {
        println!("It's seven");
    } else if let Some(value) = number {
        println!("It's {}", value);
    } else {
        println!("It's something else");
    }

    // example of a range
    let range = 0..=50; // inclusive range
    for number in range {
        println!("{}", number);
    }

    // function that prints each character of a string
    let s = String::from("hello");  
    for c in s.chars() {
        println!("{}", c);
    }

}
