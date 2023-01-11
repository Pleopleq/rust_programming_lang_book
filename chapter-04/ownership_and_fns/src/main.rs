fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    println!("{}", x);

    // Line 12 will throw a compile-time-error ("borrow of moved value") because the value was moved to TAKES_OWNERSHIP fn
    println!("{}", s);
}


fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int)
}