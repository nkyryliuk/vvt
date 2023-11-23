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
    hit_points: HitPoints,
    armor_class: u32,
}

impl Character {
    pub fn ability_check(&self, ability: Ability) -> i32 {
        rand::thread_rng().gen_range(1..=20) + ability.get_modifier()
    }

    pub fn skill_check(&self, skill: Skill) -> i32 {
        let proficiency = match skill.kind {
            SkillType::Acrobatics => &self.proficiencies.acrobatics,
            SkillType::AnimalHandling => &self.proficiencies.animal_handling,
            SkillType::Arcana => &self.proficiencies.arcana,
            SkillType::Athletics => &self.proficiencies.athletics,
            SkillType::Deception => &self.proficiencies.deception,
            SkillType::History => &self.proficiencies.history,
            SkillType::Insight => &self.proficiencies.insight,
            SkillType::Intimidation => &self.proficiencies.intimidation,
            SkillType::Investigation => &self.proficiencies.investigation,
            SkillType::Medicine => &self.proficiencies.medicine,
            SkillType::Nature => &self.proficiencies.nature,
            SkillType::Perception => &self.proficiencies.perception,
            SkillType::Performance => &self.proficiencies.performance,
            SkillType::Persuasion => &self.proficiencies.persuasion,
            SkillType::Religion => &self.proficiencies.religion,
            SkillType::SleightOfHand => &self.proficiencies.sleight_of_hand,
            SkillType::Stealth => &self.proficiencies.stealth,
            SkillType::Survival => &self.proficiencies.survival,
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

#[derive(Debug, PartialEq)]
pub struct HitPoints {
    current: u32,
    max: u32,
    temporary: u32,
}

pub struct Skills {
    acrobatics: Skill,
    animal_handling: Skill,
    arcana: Skill,
    athletics: Skill,
    deception: Skill,
    history: Skill,
    insight: Skill,
    intimidation: Skill,
    investigation: Skill,
    medicine: Skill,
    nature: Skill,
    perception: Skill,
    performance: Skill,
    persuasion: Skill,
    religion: Skill,
    sleight_of_hand: Skill,
    stealth: Skill,
    survival: Skill,
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
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
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
                acrobatics: Skill {
                    kind: SkillType::Acrobatics,
                    value: 0,
                },
                animal_handling: Skill {
                    kind: SkillType::AnimalHandling,
                    value: 0,
                },
                arcana: Skill {
                    kind: SkillType::Arcana,
                    value: 0,
                },
                athletics: Skill {
                    kind: SkillType::Athletics,
                    value: 0,
                },
                deception: Skill {
                    kind: SkillType::Deception,
                    value: 0,
                },
                history: Skill {
                    kind: SkillType::History,
                    value: 0,
                },
                insight: Skill {
                    kind: SkillType::Insight,
                    value: 0,
                },
                intimidation: Skill {
                    kind: SkillType::Intimidation,
                    value: 0,
                },
                investigation: Skill {
                    kind: SkillType::Investigation,
                    value: 0,
                },
                medicine: Skill {
                    kind: SkillType::Medicine,
                    value: 0,
                },
                nature: Skill {
                    kind: SkillType::Nature,
                    value: 0,
                },
                perception: Skill {
                    kind: SkillType::Perception,
                    value: 0,
                },
                performance: Skill {
                    kind: SkillType::Performance,
                    value: 0,
                },
                persuasion: Skill {
                    kind: SkillType::Persuasion,
                    value: 0,
                },
                religion: Skill {
                    kind: SkillType::Religion,
                    value: 0,
                },
                sleight_of_hand: Skill {
                    kind: SkillType::SleightOfHand,
                    value: 0,
                },
                stealth: Skill {
                    kind: SkillType::Stealth,
                    value: 0,
                },
                survival: Skill {
                    kind: SkillType::Survival,
                    value: 0,
                },
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
            hit_points: HitPoints {
                current: 10,
                max: 10,
                temporary: 0,
            },
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
        assert_eq!(
            character.skills.acrobatics,
            Skill {
                kind: SkillType::Acrobatics,
                value: 0
            }
        );
        assert_eq!(character.proficiencies.acrobatics, Proficiency::Absent);
        assert_eq!(
            character.hit_points,
            HitPoints {
                current: 10,
                max: 10,
                temporary: 0,
            }
        );
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
                acrobatics: Skill {
                    kind: SkillType::Acrobatics,
                    value: 0,
                },
                animal_handling: Skill {
                    kind: SkillType::AnimalHandling,
                    value: 0,
                },
                arcana: Skill {
                    kind: SkillType::Arcana,
                    value: 0,
                },
                athletics: Skill {
                    kind: SkillType::Athletics,
                    value: 0,
                },
                deception: Skill {
                    kind: SkillType::Deception,
                    value: 0,
                },
                history: Skill {
                    kind: SkillType::History,
                    value: 0,
                },
                insight: Skill {
                    kind: SkillType::Insight,
                    value: 0,
                },
                intimidation: Skill {
                    kind: SkillType::Intimidation,
                    value: 0,
                },
                investigation: Skill {
                    kind: SkillType::Investigation,
                    value: 0,
                },
                medicine: Skill {
                    kind: SkillType::Medicine,
                    value: 0,
                },
                nature: Skill {
                    kind: SkillType::Nature,
                    value: 0,
                },
                perception: Skill {
                    kind: SkillType::Perception,
                    value: 0,
                },
                performance: Skill {
                    kind: SkillType::Performance,
                    value: 0,
                },
                persuasion: Skill {
                    kind: SkillType::Persuasion,
                    value: 0,
                },
                religion: Skill {
                    kind: SkillType::Religion,
                    value: 0,
                },
                sleight_of_hand: Skill {
                    kind: SkillType::SleightOfHand,
                    value: 0,
                },
                stealth: Skill {
                    kind: SkillType::Stealth,
                    value: 0,
                },
                survival: Skill {
                    kind: SkillType::Survival,
                    value: 0,
                },
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
            hit_points: HitPoints {
                current: 10,
                max: 10,
                temporary: 10,
            },
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
