use rand::{thread_rng, prelude::SliceRandom};
use std::io;

fn main() {
    let mut length_str = String::new();
    let mut special_chars = String::new();
    let mut complexity = String::new();

    // Step 1: Ask for the length of the password
    println!("Enter the desired length of the password:");
    io::stdin().read_line(&mut length_str).unwrap();
    let length: usize = length_str.trim().parse().unwrap_or(8); // Default to 8

    // Step 2: Ask if they want to include special characters
    println!("Do you want to include special characters? (yes/no):");
    io::stdin().read_line(&mut special_chars).unwrap();
    let include_specials = special_chars.trim().eq("yes");

    // Step 3: Ask for complexity (This is a placeholder for actual complexity features)
    println!("Enter desired complexity level (1-3):");
    io::stdin().read_line(&mut complexity).unwrap();
    let _complexity_level: i32 = complexity.trim().parse().unwrap_or(1); // Not used in this example

    // Step 4: (Implicit in this case) Using rand for password generation

    // Step 5: Generate and display the password
    let password = generate_password(length, include_specials);
    println!("Generated Password: {}", password);
}

fn generate_password(length: usize, include_specials: bool) -> String {
    let mut rng = thread_rng();
    let mut password = String::new();

    // Define a set of special characters
    let specials = "!@#$%^&*()_-+=[]{}|;:,.<>?";

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

