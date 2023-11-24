use std::io::{self, Write};

use crate::models::{
    Abilities, Ability, Character, Class, ClassDetails, HitPoints, Inventory, Proficiency, Race,
    SavingThrow, Skill, SkillProficiencies, SkillType, Skills,
};
use clap::Parser;

pub struct State {
    characters: Vec<Character>,
}

impl State {
    pub fn new() -> Self {
        Self {
            characters: Vec::new(),
        }
    }
}

pub struct TerminalInterface {
    pub state: State,
}

impl TerminalInterface {
    pub fn new() -> Self {
        Self {
            state: State {
                characters: Vec::new(),
            },
        }
    }

    pub fn run(&mut self) {
        loop {
            println!("Choose an option:");
            println!("1. Create a new character");
            println!("2. List all characters");
            println!("3. Exit");
            print!("Enter your choice: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let choice = input.trim().parse::<u32>().unwrap();
            match choice {
                1 => self.add_character(),
                2 => self.list_characters(),
                3 => break,
                _ => println!("Invalid choice"),
            }
        }
    }

    fn add_character(&mut self) {
        let mut name_input = String::new();
        print!("Enter character name: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut name_input).unwrap();
        let name = name_input.trim().to_string();

        let mut level_input = String::new();
        print!("Enter a character level: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut level_input).unwrap();
        let level = level_input.trim().parse::<u32>().unwrap();

        let mut hp_input = String::new();
        print!("Enter a character hitpoints: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut hp_input).unwrap();
        let hp = hp_input.trim().parse::<u32>().unwrap();

        let mut race_input = String::new();
        print!("Enter a character race: (Human, Elf, Dwarf, HalfOrc, Halfling)");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut race_input).unwrap();
        let race = match race_input.trim().to_string().as_str() {
            "Human" => Race::Human,
            "Elf" => Race::Elf,
            "Dwarf" => Race::Dwarf,
            "HalfOrc" => Race::HalfOrc,
            "Halfling" => Race::Halfling,

            _ => todo!(),
        };

        let mut armor_class_input = String::new();
        print!("Enter a character armor class: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut armor_class_input).unwrap();
        let armor_class = armor_class_input.trim().parse::<u32>().unwrap();

        let mut class_input = String::new();
        print!("Enter a character class: (Fighter, Wizard, Rogue, Cleric)");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut class_input).unwrap();
        let class_type = match class_input.trim().to_string().as_str() {
            "Fighter" => Class::Fighter,
            "Wizard" => Class::Wizard,
            "Rogue" => Class::Rogue,
            "Cleric" => Class::Cleric,

            _ => todo!(),
        };
        let class = ClassDetails {
            name: class_input.trim().to_string(),
            hit_dice: {
                match class_type {
                    Class::Fighter => 10,
                    Class::Wizard => 6,
                    Class::Rogue => 8,
                    Class::Cleric => 8,
                    Class::Bard => todo!(),
                    Class::Druid => todo!(),
                    Class::Monk => todo!(),
                    Class::Paladin => todo!(),
                    Class::Ranger => todo!(),
                    Class::Sorcerer => todo!(),
                    Class::Warlock => todo!(),
                    Class::Barbarian => todo!(),
                }
            },
            saving_throws: {
                match class_type {
                    Class::Fighter => vec![SavingThrow::Strength, SavingThrow::Constitution],
                    Class::Wizard => vec![SavingThrow::Intelligence, SavingThrow::Wisdom],
                    Class::Rogue => vec![SavingThrow::Dexterity, SavingThrow::Intelligence],
                    Class::Cleric => vec![SavingThrow::Wisdom, SavingThrow::Charisma],
                    Class::Bard => todo!(),
                    Class::Druid => todo!(),
                    Class::Monk => todo!(),
                    Class::Paladin => todo!(),
                    Class::Ranger => todo!(),
                    Class::Sorcerer => todo!(),
                    Class::Warlock => todo!(),
                    Class::Barbarian => todo!(),
                }
            },
        };

        let mut abilities = Abilities {
            strength: Ability { value: 0 },
            dexterity: Ability { value: 0 },
            constitution: Ability { value: 0 },
            intelligence: Ability { value: 0 },
            wisdom: Ability { value: 0 },
            charisma: Ability { value: 0 },
        };
        let abilities_list = vec![
            "strength",
            "dexterity",
            "constitution",
            "intelligence",
            "wisdom",
            "charisma",
        ];
        for ability in abilities_list {
            let mut input = String::new();
            print!("Enter a value for {}: ", ability);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let value = input.trim().parse::<u32>().unwrap();
            match ability {
                "strength" => abilities.strength.value = value,
                "dexterity" => abilities.dexterity.value = value,
                "constitution" => abilities.constitution.value = value,
                "intelligence" => abilities.intelligence.value = value,
                "wisdom" => abilities.wisdom.value = value,
                "charisma" => abilities.charisma.value = value,
                _ => println!("Invalid ability"),
            }
        }

        let skill_list = vec![
            "acrobatics",
            "animal_handling",
            "arcana",
            "athletics",
            "deception",
            "history",
            "insight",
            "intimidation",
            "investigation",
            "medicine",
            "nature",
            "perception",
            "performance",
            "persuasion",
            "religion",
            "sleight_of_hand",
            "stealth",
            "survival",
        ];
        let mut skills = Skills {
            acrobatics: Skill {
                value: 0,
                kind: SkillType::Acrobatics,
            },
            animal_handling: Skill {
                value: 0,
                kind: SkillType::AnimalHandling,
            },
            arcana: Skill {
                value: 0,
                kind: SkillType::Arcana,
            },
            athletics: Skill {
                value: 0,
                kind: SkillType::Athletics,
            },
            deception: Skill {
                value: 0,
                kind: SkillType::Deception,
            },
            history: Skill {
                value: 0,
                kind: SkillType::History,
            },
            insight: Skill {
                value: 0,
                kind: SkillType::Insight,
            },
            intimidation: Skill {
                value: 0,
                kind: SkillType::Intimidation,
            },
            investigation: Skill {
                value: 0,
                kind: SkillType::Investigation,
            },
            medicine: Skill {
                value: 0,
                kind: SkillType::Medicine,
            },
            nature: Skill {
                value: 0,
                kind: SkillType::Nature,
            },
            perception: Skill {
                value: 0,
                kind: SkillType::Perception,
            },
            performance: Skill {
                value: 0,
                kind: SkillType::Performance,
            },
            persuasion: Skill {
                value: 0,
                kind: SkillType::Persuasion,
            },
            religion: Skill {
                value: 0,
                kind: SkillType::Religion,
            },
            sleight_of_hand: Skill {
                value: 0,
                kind: SkillType::SleightOfHand,
            },
            stealth: Skill {
                value: 0,
                kind: SkillType::Stealth,
            },
            survival: Skill {
                value: 0,
                kind: SkillType::Survival,
            },
        };
        for skill in skill_list {
            let mut input = String::new();
            print!("Enter a value for {}: ", skill);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let value = input.trim().parse::<u32>().unwrap();
            match skill {
                "acrobatics" => skills.acrobatics.value = value,
                "animal_handling" => skills.animal_handling.value = value,
                "arcana" => skills.arcana.value = value,
                "athletics" => skills.athletics.value = value,
                "deception" => skills.deception.value = value,
                "history" => skills.history.value = value,
                "insight" => skills.insight.value = value,
                "intimidation" => skills.intimidation.value = value,
                "investigation" => skills.investigation.value = value,
                "medicine" => skills.medicine.value = value,
                "nature" => skills.nature.value = value,
                "perception" => skills.perception.value = value,
                "performance" => skills.performance.value = value,
                "persuasion" => skills.persuasion.value = value,
                "religion" => skills.religion.value = value,
                "sleight_of_hand" => skills.sleight_of_hand.value = value,
                "stealth" => skills.stealth.value = value,
                "survival" => skills.survival.value = value,
                _ => println!("Invalid skill"),
            }
        }

        let skill_proficiencies_list = vec![
            "acrobatics",
            "animal_handling",
            "arcana",
            "athletics",
            "deception",
            "history",
            "insight",
            "intimidation",
            "investigation",
            "medicine",
            "nature",
            "perception",
            "performance",
            "persuasion",
            "religion",
            "sleight_of_hand",
            "stealth",
            "survival",
        ];
        let mut skill_proficiencies = SkillProficiencies {
            acrobatics: Proficiency::NotProficient,
            animal_handling: Proficiency::NotProficient,
            arcana: Proficiency::NotProficient,
            athletics: Proficiency::NotProficient,
            deception: Proficiency::NotProficient,
            history: Proficiency::NotProficient,
            insight: Proficiency::NotProficient,
            intimidation: Proficiency::NotProficient,
            investigation: Proficiency::NotProficient,
            medicine: Proficiency::NotProficient,
            nature: Proficiency::NotProficient,
            perception: Proficiency::NotProficient,
            performance: Proficiency::NotProficient,
            persuasion: Proficiency::NotProficient,
            religion: Proficiency::NotProficient,
            sleight_of_hand: Proficiency::NotProficient,
            stealth: Proficiency::NotProficient,
            survival: Proficiency::NotProficient,
        };
        for skill_proficiency in skill_proficiencies_list {
            let mut input = String::new();
            print!("Is {} proficient? (y/n/h/e): ", skill_proficiency);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let value = input.trim().to_string();
            match value.as_str() {
                "y" => match skill_proficiency {
                    "acrobatics" => skill_proficiencies.acrobatics = Proficiency::Proficient,
                    "animal_handling" => {
                        skill_proficiencies.animal_handling = Proficiency::Proficient
                    }
                    "arcana" => skill_proficiencies.arcana = Proficiency::Proficient,
                    "athletics" => skill_proficiencies.athletics = Proficiency::Proficient,
                    "deception" => skill_proficiencies.deception = Proficiency::Proficient,
                    "history" => skill_proficiencies.history = Proficiency::Proficient,
                    "insight" => skill_proficiencies.insight = Proficiency::Proficient,
                    "intimidation" => skill_proficiencies.intimidation = Proficiency::Proficient,
                    "investigation" => skill_proficiencies.investigation = Proficiency::Proficient,
                    "medicine" => skill_proficiencies.medicine = Proficiency::Proficient,
                    "nature" => skill_proficiencies.nature = Proficiency::Proficient,
                    "perception" => skill_proficiencies.perception = Proficiency::Proficient,
                    "performance" => skill_proficiencies.performance = Proficiency::Proficient,
                    "persuasion" => skill_proficiencies.persuasion = Proficiency::Proficient,
                    "religion" => skill_proficiencies.religion = Proficiency::Proficient,
                    "sleight_of_hand" => {
                        skill_proficiencies.sleight_of_hand = Proficiency::Proficient
                    }
                    "stealth" => skill_proficiencies.stealth = Proficiency::Proficient,
                    "survival" => skill_proficiencies.survival = Proficiency::Proficient,
                    _ => println!("Invalid skill"),
                },

                "n" => match skill_proficiency {
                    "acrobatics" => skill_proficiencies.acrobatics = Proficiency::NotProficient,
                    "animal_handling" => {
                        skill_proficiencies.animal_handling = Proficiency::NotProficient
                    }
                    "arcana" => skill_proficiencies.arcana = Proficiency::NotProficient,
                    "athletics" => skill_proficiencies.athletics = Proficiency::NotProficient,
                    "deception" => skill_proficiencies.deception = Proficiency::NotProficient,
                    "history" => skill_proficiencies.history = Proficiency::NotProficient,
                    "insight" => skill_proficiencies.insight = Proficiency::NotProficient,
                    "intimidation" => skill_proficiencies.intimidation = Proficiency::NotProficient,
                    "investigation" => {
                        skill_proficiencies.investigation = Proficiency::NotProficient
                    }
                    "medicine" => skill_proficiencies.medicine = Proficiency::NotProficient,
                    "nature" => skill_proficiencies.nature = Proficiency::NotProficient,
                    "perception" => skill_proficiencies.perception = Proficiency::NotProficient,
                    "performance" => skill_proficiencies.performance = Proficiency::NotProficient,
                    "persuasion" => skill_proficiencies.persuasion = Proficiency::NotProficient,
                    "religion" => skill_proficiencies.religion = Proficiency::NotProficient,
                    "sleight_of_hand" => {
                        skill_proficiencies.sleight_of_hand = Proficiency::NotProficient
                    }
                    "stealth" => skill_proficiencies.stealth = Proficiency::NotProficient,
                    "survival" => skill_proficiencies.survival = Proficiency::NotProficient,
                    _ => println!("Invalid skill"),
                },

                "h" => match skill_proficiency {
                    "acrobatics" => skill_proficiencies.acrobatics = Proficiency::HalfProficient,
                    "animal_handling" => {
                        skill_proficiencies.animal_handling = Proficiency::HalfProficient
                    }
                    "arcana" => skill_proficiencies.arcana = Proficiency::HalfProficient,
                    "athletics" => skill_proficiencies.athletics = Proficiency::HalfProficient,
                    "deception" => skill_proficiencies.deception = Proficiency::HalfProficient,
                    "history" => skill_proficiencies.history = Proficiency::HalfProficient,
                    "insight" => skill_proficiencies.insight = Proficiency::HalfProficient,
                    "intimidation" => {
                        skill_proficiencies.intimidation = Proficiency::HalfProficient
                    }
                    "investigation" => {
                        skill_proficiencies.investigation = Proficiency::HalfProficient
                    }
                    "medicine" => skill_proficiencies.medicine = Proficiency::HalfProficient,
                    "nature" => skill_proficiencies.nature = Proficiency::HalfProficient,
                    "perception" => skill_proficiencies.perception = Proficiency::HalfProficient,
                    "performance" => skill_proficiencies.performance = Proficiency::HalfProficient,
                    "persuasion" => skill_proficiencies.persuasion = Proficiency::HalfProficient,
                    "religion" => skill_proficiencies.religion = Proficiency::HalfProficient,
                    "sleight_of_hand" => {
                        skill_proficiencies.sleight_of_hand = Proficiency::HalfProficient
                    }
                    "stealth" => skill_proficiencies.stealth = Proficiency::HalfProficient,
                    "survival" => skill_proficiencies.survival = Proficiency::HalfProficient,
                    _ => println!("Invalid skill"),
                },

                "e" => match skill_proficiency {
                    "acrobatics" => skill_proficiencies.acrobatics = Proficiency::Expertise,
                    "animal_handling" => {
                        skill_proficiencies.animal_handling = Proficiency::Expertise
                    }
                    "arcana" => skill_proficiencies.arcana = Proficiency::Expertise,
                    "athletics" => skill_proficiencies.athletics = Proficiency::Expertise,
                    "deception" => skill_proficiencies.deception = Proficiency::Expertise,
                    "history" => skill_proficiencies.history = Proficiency::Expertise,
                    "insight" => skill_proficiencies.insight = Proficiency::Expertise,
                    "intimidation" => skill_proficiencies.intimidation = Proficiency::Expertise,
                    "investigation" => skill_proficiencies.investigation = Proficiency::Expertise,
                    "medicine" => skill_proficiencies.medicine = Proficiency::Expertise,
                    "nature" => skill_proficiencies.nature = Proficiency::Expertise,
                    "perception" => skill_proficiencies.perception = Proficiency::Expertise,
                    "performance" => skill_proficiencies.performance = Proficiency::Expertise,
                    "persuasion" => skill_proficiencies.persuasion = Proficiency::Expertise,
                    "religion" => skill_proficiencies.religion = Proficiency::Expertise,
                    "sleight_of_hand" => {
                        skill_proficiencies.sleight_of_hand = Proficiency::Expertise
                    }
                    "stealth" => skill_proficiencies.stealth = Proficiency::Expertise,
                    "survival" => skill_proficiencies.survival = Proficiency::Expertise,
                    _ => println!("Invalid skill"),
                },

                _ => println!("Invalid skill"),
            }
        }

        let character = Character {
            name,
            race,
            level,
            class,
            abilities,
            skills,
            armor_class,
            proficiencies: skill_proficiencies,
            hit_points: HitPoints {
                current: hp,
                max: hp,
                temporary: 0,
            },
            inventory: Inventory { items: Vec::new() },
        };
        self.state.characters.push(character);
    }

    fn list_characters(&self) {
        for character in &self.state.characters {
            println!("{:#?}", character);
        }
    }
}
