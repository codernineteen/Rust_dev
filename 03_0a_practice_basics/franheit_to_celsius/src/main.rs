use std::io;

fn main() {
    println!("Please input fahranheit");
    let mut fahrenheit = String::new();

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read");
    
    let fahrenheit = fahrenheit.trim().parse().expect("Input should be number");
    let celsius = convert_formula(fahrenheit);
    println!("Fahrenheit is {}, Celsius corresponding to F is {}", fahrenheit, celsius);
}

fn convert_formula(fahranheit: i32) -> i32 {
    (fahranheit-32)*5/9
}