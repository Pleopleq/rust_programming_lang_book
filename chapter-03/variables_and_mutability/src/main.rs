fn shadowing() {
    let x: i32 = 5;
    let x: i32 = x + 1;
    let x: i32 = x * 2;

    // with "shadowing" we can re-assign values to the same var name and keeping it inmmutable
    // if we make it mutable we get a compile-time error
    println!("The value of x is: {}", x)
}


fn main() {
    // vars by default are inmutable
    // you have to use the keyword "mut" in front of the var name to make it mutable
    let mut x = 5;
    // Convention in rust to write const vars in caps with underscores between words
    const MAX_POINTS: u32 = 100_000;

    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);
    println!("Your points: {}", MAX_POINTS);

    shadowing()
}

