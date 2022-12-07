use std::io::stdin;

fn main() {
    let mut user_input: String = String::new();
    let stdin = stdin();

    println!("Celcius to Farenheit convertor");
    println!("Please input the amount to convert to farenheit: ");

    stdin.read_line(&mut user_input)
        .expect("Failed to read line");
    
    let user_input: u32 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(error) => panic!("Problem getting a non numeric value: {}", error),
    };
    
    let celcius_from_fahrenheit = (user_input as f32 * 1.8000) + 32.00 as f32;

    println!("The number of grades turned to Farenheit was: {} °F", celcius_from_fahrenheit);
}
