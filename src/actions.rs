use crate::effect::Effect;

#[derive(Clone, Debug, PartialEq)]
pub struct Attack {
    pub name: String,
    pub actor_id: u32,
    pub target_id: u32,
    pub effect: Effect,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CastSpell {
    pub name: String,
    pub actor_id: u32,
    pub target_id: u32,
    pub effect: Effect,
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
