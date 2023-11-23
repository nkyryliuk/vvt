use rand::Rng;

pub struct Character {
    name: String,
    race: Race,
    class: ClassDetails,
    level: u32,
    abilities: Abilities,
    skills: Skills,
    proficiencies: SkillProficiencies,
    inventory: Inventory,
    hit_points: u32,
    armor_class: u32,
}

impl Character {
    pub fn ability_check(&self, ability: Ability) -> i32 {
        rand::thread_rng().gen_range(1..=20) + ability.get_modifier()
    }

    pub fn skill_check(&self, skill: Skill) -> i32 {
        let proficiency = match skill.kind {
            SkillType::Strength => &self.proficiencies.acrobatics,
            SkillType::Dexterity => &self.proficiencies.animal_handling,
            SkillType::Constitution => &self.proficiencies.arcana,
            SkillType::Intelligence => &self.proficiencies.athletics,
            SkillType::Wisdom => &self.proficiencies.deception,
            SkillType::Charisma => &self.proficiencies.history,
        };

        let proficiency_bonus = match proficiency {
            Proficiency::Absent => 0,
            Proficiency::Expertise => 2 * self.level as i32,
            Proficiency::Proficiency => self.level as i32,
            Proficiency::HalfProficiency => (self.level as f32 / 2.0).floor() as i32,
        };

        rand::thread_rng().gen_range(1..=20) + skill.value as i32 + proficiency_bonus
    }

    pub fn saving_throw(&self, saving_throw: SavingThrow) -> i32 {
        let ability = match saving_throw {
            SavingThrow::Strength => &self.abilities.strength,
            SavingThrow::Dexterity => &self.abilities.dexterity,
            SavingThrow::Constitution => &self.abilities.constitution,
            SavingThrow::Intelligence => &self.abilities.intelligence,
            SavingThrow::Wisdom => &self.abilities.wisdom,
            SavingThrow::Charisma => &self.abilities.charisma,
        };

        if self.class.saving_throws.contains(&saving_throw) {
            // TODO: Add proficiency bonus
            ability.get_modifier() + self.level as i32
        } else {
            ability.get_modifier()
        }
    }
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct ClassDetails {
    name: String,
    hit_dice: u32,
    saving_throws: Vec<SavingThrow>,
}

#[derive(Debug, PartialEq)]
pub struct Ability {
    value: u32,
}

#[derive(Debug, PartialEq)]
enum SkillType {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Debug, PartialEq)]
pub struct Skill {
    kind: SkillType,
    value: u32,
}

impl Ability {
    fn get_modifier(&self) -> i32 {
        (self.value as i32 - 10) / 2
    }
}

#[derive(Debug, PartialEq)]
pub struct Abilities {
    strength: Ability,
    dexterity: Ability,
    constitution: Ability,
    intelligence: Ability,
    wisdom: Ability,
    charisma: Ability,
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
            class: ClassDetails {
                name: String::from("Fighter"),
                hit_dice: 10,
                saving_throws: vec![SavingThrow::Strength, SavingThrow::Constitution],
            },
            level: 1,
            abilities: Abilities {
                strength: Ability { value: 10 },
                dexterity: Ability { value: 10 },
                constitution: Ability { value: 10 },
                intelligence: Ability { value: 10 },
                wisdom: Ability { value: 10 },
                charisma: Ability { value: 10 },
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
        assert_eq!(
            character.class,
            ClassDetails {
                name: String::from("Fighter"),
                hit_dice: 10,
                saving_throws: vec![SavingThrow::Strength, SavingThrow::Constitution],
            }
        );
        assert_eq!(character.level, 1);
        assert_eq!(character.abilities.strength, Ability { value: 10 });
        assert_eq!(character.skills.acrobatics, 0);
        assert_eq!(character.proficiencies.acrobatics, Proficiency::Absent);
        assert_eq!(character.hit_points, 10);
        assert_eq!(character.armor_class, 10);
    }

    #[test]
    fn test_character_saving_throw() {
        let character = Character {
            name: String::from("Test Character"),
            race: Race::Human,
            class: ClassDetails {
                name: String::from("Fighter"),
                hit_dice: 10,
                saving_throws: vec![SavingThrow::Strength, SavingThrow::Constitution],
            },
            level: 1,
            abilities: Abilities {
                strength: Ability { value: 10 },
                dexterity: Ability { value: 12 },
                constitution: Ability { value: 14 },
                intelligence: Ability { value: 16 },
                wisdom: Ability { value: 18 },
                charisma: Ability { value: 20 },
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

        // Test saving throw for each ability
        assert_eq!(character.saving_throw(SavingThrow::Strength), 1);
        assert_eq!(character.saving_throw(SavingThrow::Dexterity), 1);
        assert_eq!(character.saving_throw(SavingThrow::Constitution), 3);
        assert_eq!(character.saving_throw(SavingThrow::Intelligence), 3);
        assert_eq!(character.saving_throw(SavingThrow::Wisdom), 4);
        assert_eq!(character.saving_throw(SavingThrow::Charisma), 5);
    }
}
