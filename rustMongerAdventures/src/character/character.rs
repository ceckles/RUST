use colored::*;

// Define the possible classes
pub enum CharacterClass {
    Paladin,
    Warrior,
    Cleric,
    Rogue,
}

impl CharacterClass {
    pub fn from_input(input: &str) -> Option<Self> {
        match input.trim().to_lowercase().as_str() {
            "1" | "paladin" => Some(Self::Paladin),
            "2" | "warrior" => Some(Self::Warrior),
            "3" | "cleric" => Some(Self::Cleric),
            "4" | "rogue" => Some(Self::Rogue),
            _ => None,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Self::Paladin => "Paladin",
            Self::Warrior => "Warrior",
            Self::Cleric => "Cleric",
            Self::Rogue => "Rogue",
        }
    }
}

pub struct Character {
    pub name: String,
    pub class: CharacterClass,
    pub health: u32,
    pub attack_power: u32,
    pub agility: u32,
    pub mana: u32,
    pub level: u32,
}

impl Character {
    pub fn new(name: &str, class: CharacterClass) -> Self {
        Character {
            name: name.to_string(),
            class,
            health: 100,
            attack_power: 10,
            agility: 5,
            mana: 50,
            level: 1,
        }
    }

    pub fn level_up(&mut self) {
        // Increase stats by a fixed amount or some randomization
        self.health += 20;
        self.attack_power += 5;
        self.agility += 3;
        self.mana += 10;
        self.level += 1;
        println!("{} leveled up!", self.name);
    }

    pub fn display_info(&self) {
        println!("{}", "Character Information:".truecolor(0, 255, 0)); // Custom green
        println!("{}", format!("Name: {}", self.name).truecolor(0, 255, 0)); // Custom green
        println!("{}", format!("Level: {}", self.level).truecolor(0, 255, 0)); // Custom green
        println!("{}", format!("Class: {}", self.class.to_string()).truecolor(0, 255, 0)); // Custom green
        println!("{}", format!("Health: {}", self.health).truecolor(0, 255, 0)); // Custom green
        println!("{}", format!("Attack Power: {}", self.attack_power).truecolor(0, 255, 0)); // Custom green
        println!("{}", format!("Agility: {}", self.agility).truecolor(0, 255, 0)); // Custom green
        println!("{}", format!("Mana Level: {}", self.mana).truecolor(0, 255, 0)); // Custom green
    }
}
