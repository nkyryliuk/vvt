use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum DamageKind {
    Bludgeoning,
    Piercing,
    Slashing,
    Acid,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Poison,
    Psychic,
    Radiant,
    Thunder,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Damage {
    amount: u32,
    kind: DamageKind,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum ConditionKind {
    Blinded,
    Charmed,
    Deafened,
    Exhaustion,
    Frightened,
    Grappled,
    Incapacitated,
    Invisible,
    Paralyzed,
    Petrified,
    Poisoned,
    Prone,
    Restrained,
    Stunned,
    Unconscious,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum DurationUnit {
    Rounds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Duration {
    amount: u32,
    unit: DurationUnit,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Condition {
    kind: ConditionKind,
    duration: Duration,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum BuffKind {
    Enhancement,
    Advantage,
    Disadvantage,
    TemporaryHitPoints,
    BonusAction,
    Reaction,
    Resistance,
    Immunity,
    Vulnerability,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Buff {
    kind: BuffKind,
    duration: Duration,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Effect {
    Damage(Damage),
    Condition(Condition),
    Buff(Buff),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_effect_json_serialization() {
        let damage = Damage {
            amount: 10,
            kind: DamageKind::Fire,
        };
        let effect_damage = Effect::Damage(damage);
        let serialized_damage = serde_json::to_string(&effect_damage).unwrap();
        let deserialized_damage: Effect = serde_json::from_str(&serialized_damage).unwrap();
        assert_eq!(effect_damage, deserialized_damage);

        let condition = Condition {
            kind: ConditionKind::Blinded,
            duration: Duration {
                amount: 5,
                unit: DurationUnit::Rounds,
            },
        };
        let effect_condition = Effect::Condition(condition);
        let serialized_condition = serde_json::to_string(&effect_condition).unwrap();
        let deserialized_condition: Effect = serde_json::from_str(&serialized_condition).unwrap();
        assert_eq!(effect_condition, deserialized_condition);

        let buff = Buff {
            kind: BuffKind::Enhancement,
            duration: Duration {
                amount: 3,
                unit: DurationUnit::Minutes,
            },
        };
        let effect_buff = Effect::Buff(buff);
        let serialized_buff = serde_json::to_string(&effect_buff).unwrap();
        let deserialized_buff: Effect = serde_json::from_str(&serialized_buff).unwrap();
        assert_eq!(effect_buff, deserialized_buff);
    }
}
