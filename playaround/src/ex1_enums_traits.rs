/*

Imagine Chris, a game developer, wants to create diverse characters with unique abilities.
He starts with a basic structure: enum CharacterType { Wizard, Warrior }.
How can Chris use generics to make this enum more flexible for storing different
attribute types (like i32 for health, String for name, etc.)?

*/

enum CharacterType<T> {
    Wizard {
        health: T,
        name: String,
        active: bool,
    },
    Warrior {
        health: T,
        name: String,
        active: bool,
    },
}

/*

let's consider how traits can enhance this structure.
Chris wants to define common abilities, like "power," for all character types.
How would you implement a trait named Abilities with a power method that returns a u32 value,
and then implement it for CharacterType?
*/
trait Abilities {
    fn power(&self) -> u32;
    fn toggle_active(&mut self);
    // This method will toggle the character's state between active and inactive
}

impl<T> Abilities for CharacterType<T> {
    fn power(&self) -> u32 {
        match self {
            CharacterType::Wizard {
                health,
                name: _,
                active,
            } => 50,
            CharacterType::Warrior {
                health,
                name: _,
                active,
            } => 70,
        }
    }

    fn toggle_active(&mut self) {
        match self {
            CharacterType::Wizard { active, .. } => *active = !*active,
            CharacterType::Warrior { active, .. } => *active = !*active,
        }
    }
}

/*
Now, Chris wants to add a mechanism to quickly toggle a character's state between "active" and "inactive."
How would you modify the existing `Abilities` trait to include this functionality?
 Consider adding a method and necessary data to the `CharacterType` enum to support this feature.
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_type() {
        let wizard: CharacterType<i32> = CharacterType::Wizard {
            health: 100,
            name: String::from("Gandalf"),
            active: true,
        };

        let warrior: CharacterType<String> = CharacterType::Warrior {
            health: String::from("Full"),
            name: String::from("Aragorn"),
            active: true,
        };

        match wizard {
            CharacterType::Wizard {
                health,
                name,
                active,
            } => {
                println!("Wizard {} has health {}", name, health);
            }
            _ => {}
        }

        match warrior {
            CharacterType::Warrior {
                health,
                name,
                active,
            } => {
                println!("Warrior {} has health {}", name, health);
            }
            _ => {}
        }
    }

    #[test]
    fn test_abilites() {
        let wizard: CharacterType<i32> = CharacterType::Wizard {
            health: 100,
            name: String::from("Gandalf"),
            active: true,
        };

        let warrior: CharacterType<String> = CharacterType::Warrior {
            health: String::from("Full"),
            name: String::from("Aragorn"),
            active: true,
        };

        println!("Wizard power: {}", wizard.power());
        println!("Warrior power: {}", warrior.power());
    }

    #[test]
    fn test_active() {
        let mut wizard: CharacterType<i32> = CharacterType::Wizard {
            health: 100,
            name: String::from("Gandalf"),
            active: true,
        };

        let mut warrior: CharacterType<String> = CharacterType::Warrior {
            health: String::from("Full"),
            name: String::from("Aragorn"),
            active: true,
        };

        wizard.toggle_active();
        warrior.toggle_active();

        // Check if the toggle worked
        match wizard {
            CharacterType::Wizard { active, .. } => {
                assert!(!active, "Wizard should be inactive now");
            }
            _ => {}
        }

        match warrior {
            CharacterType::Warrior { active, .. } => {
                assert!(!active, "Warrior should be inactive now");
            }
            _ => {}
        }
    }
}
