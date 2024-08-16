mod character; // Declare the module

use std::io::{self, Write}; // Import the necessary traits
use character::character::{Character, CharacterClass}; // Import the Character struct and CharacterClass enum

fn main() {
    let mut character: Option<Character> = None;

    loop {
        println!("Welcome to the Game Menu!");
        println!("1. Create a Character / View Character Info");
        println!("2. Level Up Character");
        println!("3. Start a Battle");
        println!("4. Exit");

        print!("Please choose an option (1, 2, 3, or 4): ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice = input.trim();

        match choice {
            "1" => {
                if let Some(ref mut char) = character {
                    char.display_info();
                } else {
                    create_character(&mut character);
                }
            },
            "2" => {
                if let Some(ref mut char) = character {
                    char.level_up();
                } else {
                    println!("You need to create a character first!");
                }
            },
            "3" => {
                if character.is_some() {
                    start_battle();
                } else {
                    println!("You need to create a character first!");
                }
            },
            "4" => {
                println!("Exiting the game. Goodbye!");
                break;
            },
            _ => println!("Invalid choice, please choose 1, 2, 3, or 4."),
        }
    }
}

fn create_character(character: &mut Option<Character>) {
    print!("Enter character name: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();

    println!("Select a class:");
    println!("1. Paladin");
    println!("2. Warrior");
    println!("3. Cleric");
    println!("4. Rogue");

    print!("Please choose a class (1, 2, 3, or 4): ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut class_input = String::new();
    io::stdin().read_line(&mut class_input).expect("Failed to read line");
    let class = match CharacterClass::from_input(class_input.trim()) {
        Some(cls) => cls,
        None => {
            println!("Invalid class selection, defaulting to Paladin.");
            CharacterClass::Paladin
        },
    };

    *character = Some(Character::new(name, class));
    println!("Character created successfully!");
}

fn start_battle() {
    println!("Starting the battle...");
    // Here you would add code to start a battle
}
