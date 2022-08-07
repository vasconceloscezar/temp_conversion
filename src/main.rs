use std::io;

fn main() {
    let mut option: u32 = 1;

    while option != 0 {
        println!("Which conversion you wanna make?");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("Press any other key to exit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        option = convert_input_to_number(input);

        match option {
            1 => {
                println!("Enter the farhrenheit temperature");
                let input = get_user_input();
                let fahrenheit = convert_input_to_number(input);
                let celsius = convert_to_celsius(fahrenheit);
                println!("{} Fahrenheit is {} Celsius", fahrenheit, celsius);
            }
            2 => {
                println!("Enter the celsius temperature");
                let input = get_user_input();
                let celsius = convert_input_to_number(input);
                let fahrenheit = convert_to_fahrenheit(celsius);
                println!("{} Celsius is {} Fahrenheit", celsius, fahrenheit);
            }
            _ => {
                println!("Exiting");
                option = 0;
            }
        }
        if option != 0 {
            println!("Do you wanna make another conversion? (y/n)");
            let input = get_user_input();
            if input == "n" {
                option = 0;
            }
        }
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

fn convert_input_to_number(input: String) -> u32 {
    let input = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    input
}

fn convert_to_celsius(fahrenheit: u32) -> f64 {
    (fahrenheit as f64 - 32.0) * 5.0 / 9.0
}
fn convert_to_fahrenheit(celsius: u32) -> f64 {
    celsius as f64 * 9.0 / 5.0 + 32.0
}
