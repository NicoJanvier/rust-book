use std::io;

fn main() {
    println!("Fahrenheit:");
    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line.");
    
    let temperature: u32 = temperature.trim().parse().expect("Not a number");
    let temperature = (temperature - 32) * 5/9;
    println!("Celsius: {temperature}");
}
