use rand::Rng;

#[derive(Debug, PartialEq)]
pub struct Character {
    pub name: String,
    pub race: Race,
    pub class: ClassDetails,
    pub level: u32,
    pub abilities: Abilities,
    pub skills: Skills,
    pub proficiencies: SkillProficiencies,
    pub inventory: Inventory,
    pub hit_points: HitPoints,
    pub armor_class: u32,
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
            Proficiency::NotProficient => 0,
            Proficiency::Expertise => 2 * self.level as i32,
            Proficiency::Proficient => self.level as i32,
            Proficiency::HalfProficient => (self.level as f32 / 2.0).floor() as i32,
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
    pub current: u32,
    pub max: u32,
    pub temporary: u32,
}

#[derive(Debug, PartialEq)]
pub struct Skills {
    pub acrobatics: Skill,
    pub animal_handling: Skill,
    pub arcana: Skill,
    pub athletics: Skill,
    pub deception: Skill,
    pub history: Skill,
    pub insight: Skill,
    pub intimidation: Skill,
    pub investigation: Skill,
    pub medicine: Skill,
    pub nature: Skill,
    pub perception: Skill,
    pub performance: Skill,
    pub persuasion: Skill,
    pub religion: Skill,
    pub sleight_of_hand: Skill,
    pub stealth: Skill,
    pub survival: Skill,
}

#[derive(Debug, PartialEq)]
pub enum Proficiency {
    NotProficient,
    Expertise,
    Proficient,
    HalfProficient,
}

#[derive(Debug, PartialEq)]
pub struct SkillProficiencies {
    pub acrobatics: Proficiency,
    pub animal_handling: Proficiency,
    pub arcana: Proficiency,
    pub athletics: Proficiency,
    pub deception: Proficiency,
    pub history: Proficiency,
    pub insight: Proficiency,
    pub intimidation: Proficiency,
    pub investigation: Proficiency,
    pub medicine: Proficiency,
    pub nature: Proficiency,
    pub perception: Proficiency,
    pub performance: Proficiency,
    pub persuasion: Proficiency,
    pub religion: Proficiency,
    pub sleight_of_hand: Proficiency,
    pub stealth: Proficiency,
    pub survival: Proficiency,
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
    pub name: String,
    pub hit_dice: u32,
    pub saving_throws: Vec<SavingThrow>,
}

#[derive(Debug, PartialEq)]
pub struct Ability {
    pub value: u32,
}

#[derive(Debug, PartialEq)]
pub enum SkillType {
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
    pub kind: SkillType,
    pub value: u32,
}

impl Ability {
    fn get_modifier(&self) -> i32 {
        (self.value as i32 - 10) / 2
    }
}

#[derive(Debug, PartialEq)]
pub struct Abilities {
    pub strength: Ability,
    pub dexterity: Ability,
    pub constitution: Ability,
    pub intelligence: Ability,
    pub wisdom: Ability,
    pub charisma: Ability,
}

#[derive(Debug, PartialEq)]
pub struct Inventory {
    pub items: Vec<Item>,
}

#[derive(Debug, PartialEq)]
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
        assert_eq!(
            character.proficiencies.acrobatics,
            Proficiency::NotProficient
        );
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
