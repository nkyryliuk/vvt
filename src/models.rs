pub struct Character {
    name: String,
    race: Race,
    class: Class,
    level: u32,
    abilities: Abilities,
    skills: Skills,
    proficiencies: SkillProficiencies,
    inventory: Inventory,
    hit_points: u32,
    armor_class: u32,
}

pub struct Skills {
    acrobatics: u32,
    animal_handling: u32,
    arcana: u32,
    athletics: u32,
    deception: u32,
    history: u32,
    insight: u32,
    intimidation: u32,
    investigation: u32,
    medicine: u32,
    nature: u32,
    perception: u32,
    performance: u32,
    persuasion: u32,
    religion: u32,
    sleight_of_hand: u32,
    stealth: u32,
    survival: u32,
}

#[derive(Debug, PartialEq)]
pub enum Proficiency {
    Absent,
    Expertise,
    Proficiency,
    HalfProficiency,
}

pub struct SkillProficiencies {
    acrobatics: Proficiency,
    animal_handling: Proficiency,
    arcana: Proficiency,
    athletics: Proficiency,
    deception: Proficiency,
    history: Proficiency,
    insight: Proficiency,
    intimidation: Proficiency,
    investigation: Proficiency,
    medicine: Proficiency,
    nature: Proficiency,
    perception: Proficiency,
    performance: Proficiency,
    persuasion: Proficiency,
    religion: Proficiency,
    sleight_of_hand: Proficiency,
    stealth: Proficiency,
    survival: Proficiency,
}

pub enum SavingThrow {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Debug, PartialEq)]
pub enum Race {
    Human,
    Elf,
    Dwarf,
    Halfling,
    Gnome,
    HalfElf,
    HalfOrc,
    Tiefling,
}

#[derive(Debug, PartialEq)]
pub enum Class {
    Fighter,
    Wizard,
    Rogue,
    Cleric,
    Barbarian,
    Bard,
    Druid,
    Monk,
    Paladin,
    Ranger,
    Sorcerer,
    Warlock,
}

pub struct ClassDetails {
    name: String,
    hit_dice: u32,
    saving_throws: Vec<SavingThrow>,
}

pub struct Abilities {
    strength: u32,
    dexterity: u32,
    constitution: u32,
    intelligence: u32,
    wisdom: u32,
    charisma: u32,
}

pub struct Inventory {
    items: Vec<Item>,
}

pub struct Item {
    name: String,
    quantity: u32,
}

mod tests {
    use super::*;
    #[test]
    fn test_character_creation() {
        let character = Character {
            name: String::from("Test Character"),
            race: Race::Human,
            class: Class::Fighter,
            level: 1,
            abilities: Abilities {
                strength: 10,
                dexterity: 10,
                constitution: 10,
                intelligence: 10,
                wisdom: 10,
                charisma: 10,
            },
            skills: Skills {
                acrobatics: 0,
                animal_handling: 0,
                arcana: 0,
                athletics: 0,
                deception: 0,
                history: 0,
                insight: 0,
                intimidation: 0,
                investigation: 0,
                medicine: 0,
                nature: 0,
                perception: 0,
                performance: 0,
                persuasion: 0,
                religion: 0,
                sleight_of_hand: 0,
                stealth: 0,
                survival: 0,
            },
            proficiencies: SkillProficiencies {
                acrobatics: Proficiency::Absent,
                animal_handling: Proficiency::Absent,
                arcana: Proficiency::Absent,
                athletics: Proficiency::Absent,
                deception: Proficiency::Absent,
                history: Proficiency::Absent,
                insight: Proficiency::Absent,
                intimidation: Proficiency::Absent,
                investigation: Proficiency::Absent,
                medicine: Proficiency::Absent,
                nature: Proficiency::Absent,
                perception: Proficiency::Absent,
                performance: Proficiency::Absent,
                persuasion: Proficiency::Absent,
                religion: Proficiency::Absent,
                sleight_of_hand: Proficiency::Absent,
                stealth: Proficiency::Absent,
                survival: Proficiency::Absent,
            },
            inventory: Inventory { items: vec![] },
            hit_points: 10,
            armor_class: 10,
        };

        assert_eq!(character.name, "Test Character");
        assert_eq!(character.race, Race::Human);
        assert_eq!(character.class, Class::Fighter);
        assert_eq!(character.level, 1);
        assert_eq!(character.abilities.strength, 10);
        assert_eq!(character.skills.acrobatics, 0);
        assert_eq!(character.proficiencies.acrobatics, Proficiency::Absent);
        assert_eq!(character.hit_points, 10);
        assert_eq!(character.armor_class, 10);
    }
}
