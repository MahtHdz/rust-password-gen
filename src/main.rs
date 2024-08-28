// use std::fs::File;
use std::io::{self, Write};
use rand::{thread_rng, prelude::SliceRandom};

fn main() -> io::Result<()> {
    // Step 1: Print the banner
    print_banner_from_file();

    // Rest of your main function code
    let mut length_str = String::new();
    let mut special_chars = String::new();
    let mut complexity = String::new();

    // Step 1: Ask for the length of the password
    print!("Enter the desired length of the password: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before input
    io::stdin().read_line(&mut length_str).unwrap();
    let length: usize = length_str.trim().parse().unwrap_or(8); // Default to 8

    // Step 2: Ask if they want to include special characters
    print!("Do you want to include special characters? [Y/n]: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before input
    io::stdin().read_line(&mut special_chars).unwrap();
    let include_specials = match special_chars.trim() {
        "Y" | "y" => true,
        _ => false,
    };

    // Step 3: Ask for complexity level
    print!("Enter desired complexity level (1-3): ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before input
    io::stdin().read_line(&mut complexity).unwrap();
    let complexity_level: i32 = complexity.trim().parse().unwrap_or(1);

    // Step 4: Generate and display the password
    let password = generate_password(length, include_specials, complexity_level);
    println!("Generated Password: {}", password);

    Ok(())
}

fn print_banner_from_file() {
    // If no file path is provided, use the embedded banner
    let banner: &str = include_str!("../assets/banner.ban");
    // Use the `banner` string as needed.
    println!("{}", banner);
}

fn generate_password(length: usize, include_specials: bool, complexity_level: i32) -> String {
    let mut rng = thread_rng();
    let mut password = String::new();

    // Define sets of special characters based on complexity level
    let specials_level_1 = "!@#$%^&*";
    let specials_level_2 = specials_level_1.to_string() + "()[]{}_+-";
    let specials_level_3 = specials_level_2.to_string() + "=|;:,.<>?";

    // Select the appropriate set of special characters
    let specials = match complexity_level {
        1 => specials_level_1,
        2 => specials_level_2.as_str(),
        3 => specials_level_3.as_str(),
        _ => specials_level_1, // Default to level 1 if input is out of range
    };

    // Create a vector that will hold all possible characters
    let mut all_chars: Vec<char> = Vec::new();
    all_chars.extend('a'..='z');
    all_chars.extend('A'..='Z');
    all_chars.extend('0'..='9');
    if include_specials {
        all_chars.extend(specials.chars());
    }

    // Generate password
    for _ in 0..length {
        let &random_char = all_chars.choose(&mut rng).unwrap();
        password.push(random_char);
    }

    password
}
