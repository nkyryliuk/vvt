use serde::Serialize;

use crate::effect::Effect;

#[derive(Clone, Debug, PartialEq)]
pub struct Attack {
    pub name: String,
    pub actor_id: u32,
    pub target_id: u32,
    pub effect: Effect,
}

// TODO: Add support for targeting a point on the map. Maybe move target to the effect level?
#[derive(Clone, Debug, PartialEq)]
pub enum Target {
    Actor(u32),
    Point(f32, f32),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Shape {
    NoShape,
    Cone {
        length: f32,
        angle: f32,
    },
    Cube {
        length: f32,
        width: f32,
        height: f32,
    },
    Cylinder {
        radius: f32,
        height: f32,
    },
    Sphere {
        radius: f32,
    },
    Line {
        length: f32,
    },
    Point,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CastSpell {
    pub name: String,
    pub actor_id: u32,
    pub target_id: u32,
    pub effect: Effect,
    pub shape: Shape,
}

pub enum Action {
    Attack(Attack),
    CastSpell(CastSpell),
    Dash,
    Disengage,
    Dodge,
    Help,
    Hide,
    Ready,
    Search,
    UseObject,
}

#[cfg(test)]
mod tests {
    use crate::{
        effect::{Damage, DamageKind},
        models::Dice,
    };

    use super::*;

    #[test]
    fn test_construct_fireball_spell_action() {
        let name = String::from("Fireball");
        let actor_id = 1;
        let target_id = 2;
        let effect = Effect::Damage(Damage {
            dice: Dice { count: 8, sides: 6 },
            kind: DamageKind::Fire,
        });
        let shape = Shape::Sphere { radius: 5.0 };

        let action = Action::CastSpell(CastSpell {
            name,
            actor_id,
            target_id,
            effect,
            shape,
        });

        assert!(match action {
            Action::CastSpell(_) => true,
            _ => false,
        });
    }
}
