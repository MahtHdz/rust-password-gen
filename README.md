# Rust Password Generator

This is a simple Rust script that generates random passwords.

## Overview

This project is a simple command-line tool for generating random passwords. It allows users to specify the password length, whether to include special characters, and the complexity level of the password. Additionally, the program displays a banner at startup, enhancing the user experience.

## Features

- **Password Length**: Users can define the length of the password.
- **Special Characters**: Option to include or exclude special characters in the password.
- **Complexity Levels**: Three levels of complexity, affecting the range of special characters used.
- **Banner Display**: Displays a custom banner from a file at startup.

## Getting Started

### Prerequisites

- Rust installed on your machine. You can download and install Rust from [here](https://www.rust-lang.org/).

### Installation

1. Clone the repository:

```bash
   git clone https://github.com/MahtHdz/password-generator.git
   cd password-generator
```

2. Build the project:

```bash
cargo build --release
```

3. Run the executable:

```bash
./target/release/password-generator
```

## Usage

When you run the program, it will display a banner and prompt you to enter the following:

- Password Length: Specify the desired length of the password. If no valid length is entered, the default length of 8 will be used.

- Include Special Characters: You can choose to include special characters by typing Y or y. Any other input will default to not including special characters.

- Complexity Level: Enter a complexity level between 1 and 3. Higher levels include more types of special characters.

Example of running the program:

```bash
$ ./password-generator
```

Sample interaction:

```bash
Your Banner Here!

Enter the desired length of the password: 12
Do you want to include special characters? [Y/n]: Y
Enter desired complexity level (1-3): 2
Generated Password: a9!B2cD3&4eF
```

## Banner Customization

To customize the banner:

1. Replace the contents of the assets/banner.ban file with your desired ASCII art or text.
2. The banner will be displayed every time the program starts.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.
