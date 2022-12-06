

fn main() {
    // Scalar types represent single values
    // four primary scalar types: integers, floating-point numbers, booleans and characters
    let x:f32 = 95.5;
    let y: f32 = 43.3;
    
    //numeric operations

    // addition
    let sum = 5 + 10;
    //substraction
    let difference = x - y;
    //multiplication
    let product = 4 * 30;
    //division
    let quotient = 56.7 / 32.2;
    //remainder
    let remainder = 43 % 5;

    // BOOLEANS
    let tru = true;

    let fols: bool = false;

    // CHARS
    // char types needs to be declared with single quotes and its four bytes in size
    let c = 'z';
    let z: char = 's';

    // COMPOUND TYPES
    // Compound types can group multiple values into one type
    // Exist two in Rust: Tuples and arrays.
    
    let tup: (i32, f32, u8) = (500, 5.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is {}: ", y);

    let one = tup.2;
    let five_point_four = tup.1;

    println!("Here we want to access one using their indexes: {}", one);

    println!("and the second number inside the tuple: {}", five_point_four);

    println!("Hello, world!");

    // ARRAYS

    let array: [i32; 5] = [1,2,3,4,5];

    println!("heres the array: {:?}", array);
}

