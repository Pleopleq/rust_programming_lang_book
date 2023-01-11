fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("{}", s1);

    //Error in line 11 because the value of s2 that was initialized in this scope was moved to a function
    println!("{}", s2);

    //No error because the value was given back
    println!("{}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    return some_string;
}


fn takes_and_gives_back(a_string: String) -> String {
    return a_string
}