use myfirstprogram::greet;

fn main() {
    greet();
    let _bunnies = 4;
    let _rabbits: i32 = 2;
    let (dogs, _cats) = (8, 50); // tuple pattern to initialize multiple variables at once

    println!("doges = {}", dogs);
 
    let x = 5;
    {
        let x = 99;
        println!("{}", x); // Prints "99" because the x is local to this scope, the x outside this scope is shadowed by this variable x
    }
    println!("{}", x);
}
