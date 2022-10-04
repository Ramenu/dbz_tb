use wasm_bindgen::prelude::*;
use regex::Regex;
use lazy_static::lazy_static;
use  std::fmt;

use crate::{tokenizer, effectparser::{self, EFF_RAISES, EFF_GREATLY, EFF_LOWERS, EFF_ALL, EFF_ENEMY, EFF_ATK, EFF_DEF, NULL}, modifier};

pub const MAX_SA_LEVEL : i32 = 25;

pub const SAEFFECT_NULL : u32 = 0x0;
pub const SAEFFECT_STUN : u32 = 0x1;
pub const SAEFFECT_SEAL : u32 = 0x2;
pub const SAEFFECT_ATK_ALL_ENEMIES : u32 = 0x4;
pub const SAEFFECT_STATCHANGE_ON_ENEMY : u32 = 0x8;
pub const SAEFFECT_STATCHANGE_ON_ALL_ENEMIES : u32 = 0x10;

lazy_static! {
    static ref SA_RE : Regex = Regex::new(r"^causes (low damage|damage|huge damage|extreme damage|mass damage|supreme damage|immense damage|colossal damage|mega-colossal damage)(?: to (?:the|all)? ?(?:enemy|enemies))?")
                                      .expect("Failed to compile regex");
}

// Note that modifiers can be inferred if a '%' token is encountered, so take the appropriate action 
// to add or multiply based off of that
#[wasm_bindgen]
#[derive(Default)]
pub struct SaInfo
{
    modifier : Modifier,
    effect : u32,
    atk_buff : f32,
    def_buff : f32,
    atkdef_buff_turn_count : u16, // Includes attack/def buff count in same variable
    turns_to_stunseal : u16,
    enemy_atk_reduction : f32,
    enemy_def_reduction : f32,
    enemy_atk_reduction_turn_count : u32,
    enemy_def_reduction_turn_count : u32
}

#[wasm_bindgen]
impl SaInfo
{
    #[wasm_bindgen(constructor)]
    pub fn new() -> SaInfo {
        return SaInfo { modifier: Modifier::Low, 
                        effect: SAEFFECT_NULL, 
                        atk_buff: 0.0, def_buff: 0.0, 
                        atkdef_buff_turn_count: 0, 
                        turns_to_stunseal: 0, 
                        enemy_atk_reduction: 0.0, 
                        enemy_def_reduction: 0.0, 
                        enemy_atk_reduction_turn_count: 0, 
                        enemy_def_reduction_turn_count: 0 };
    }
    #[wasm_bindgen(getter = modifier)]
    pub fn get_modifier(&self) -> Modifier {
        return self.modifier;
    }
    #[wasm_bindgen(getter = effect)]
    pub fn get_effect(&self) -> u32 {
        return self.effect;
    }
    #[wasm_bindgen(getter = atk_buff)]
    pub fn get_atk_buff(&self) -> f32 {
        return self.atk_buff;
    }
    #[wasm_bindgen(getter = def_buff)]
    pub fn get_def_buff(&self) -> f32 {
        return self.def_buff;
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Default)]
pub enum Modifier 
{
    #[default] Low,
    Damage,
    HugeDestructive,
    ExtremeMass,
    Supreme,
    Immense,
    Colossal,
    MegaColossal
}

#[cfg(debug_assertions)]
impl fmt::Display for Modifier
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self 
        {
            Modifier::Low => "low",
            Modifier::Damage => "damage",
            Modifier::HugeDestructive => "huge/destructive",
            Modifier::ExtremeMass => "extreme/mass",
            Modifier::Supreme => "supreme",
            Modifier::Immense => "immense",
            Modifier::Colossal => "colossal",
            Modifier::MegaColossal => "mega-colossal"
        };
        write!(f, "{}", s)
    }
}



#[inline]
pub fn get_sa_match(s : &str) -> Option<regex::Match> {
    return SA_RE.find(&s);
}


/// Returns a modifier wrapped in Option. If a
/// valid super attack sentence structure cannot be 
/// found then the program will most likely panic.
pub fn get_sa_modifier(s : &str) -> Option<Modifier>
{
    let modifier = SA_RE.captures(s)
                               .expect("Failed to find any match")
                               .get(1)
                               .expect("Failed to retrieve capture group 1")
                               .as_str();
    return match modifier
    {
        "low damage" => Some(Modifier::Low),
        "damage" => Some(Modifier::Damage),
        "huge damage" => Some(Modifier::HugeDestructive), // destructive damage isnt found in any of the units
        "mass damage"|"extreme damage" => Some(Modifier::ExtremeMass), 
        "supreme damage" => Some(Modifier::Supreme),
        "immense damage" => Some(Modifier::Immense),
        "colossal damage" => Some(Modifier::Colossal),
        "mega-colossal damage" => Some(Modifier::MegaColossal),
        _ => None
    };
}

/// Returns the modifier percent boost.
pub fn get_sa_modifier_atk(modifier : Modifier) -> f32
{
    const SA_MODIFIER_LOW : f32 = 1.3;
    const SA_MODIFIER_DAMAGE : f32 = 1.7;
    const SA_MODIFIER_HUGE_DESTRUCTIVE : f32 = 2.0;
    const SA_MODIFIER_EXTREME_MASS : f32 = 2.2;
    const SA_MODIFIER_SUPREME : f32 = 2.5;
    const SA_MODIFIER_IMMENSE : f32 = 2.8;
    const SA_MODIFIER_COLOSSAL : f32 = 3.0;
    const SA_MODIFIER_MEGACOLOSSAL : f32 = 3.5;

    return match modifier
    {
        Modifier::Low => (SA_MODIFIER_LOW),
        Modifier::Damage => (SA_MODIFIER_DAMAGE),
        Modifier::HugeDestructive => (SA_MODIFIER_HUGE_DESTRUCTIVE),
        Modifier::ExtremeMass => (SA_MODIFIER_EXTREME_MASS),
        Modifier::Supreme => (SA_MODIFIER_SUPREME),
        Modifier::Immense => (SA_MODIFIER_IMMENSE),
        Modifier::Colossal => (SA_MODIFIER_COLOSSAL),
        Modifier::MegaColossal => (SA_MODIFIER_MEGACOLOSSAL)
    } + 1.0;
}

/// Returns the updated attack stat with the super attack buff. This
/// function should only be called at the very end and if the unit is
/// performing a super attack. 
pub fn get_sa_atk_stat(atk : f32, modifier_dmg : f32, sa_level : i32) -> f32
{
    assert!(sa_level <= MAX_SA_LEVEL);
    return atk * modifier_dmg * sa_level as f32;
}

/// Modifies the sa struct with the proper changes to be made depending
/// on the stat effect.
fn get_sa_stat_change_eff(eff : effectparser::StatEffect, sa : &mut SaInfo)
{
    let mut boost = 0.0f32;
    if eff.get_stat_effect()&EFF_GREATLY != NULL {
        boost += modifier::EFF_GREATLY_INC_OR_DEC_MODIFIER;
    }
    if eff.get_stat_effect()&EFF_RAISES != NULL {
        boost += modifier::EFF_INC_OR_DEC_MODIFIER;
    }
    if eff.get_stat_effect()&EFF_LOWERS != NULL {
        boost -= modifier::EFF_INC_OR_DEC_MODIFIER;
    }
    if eff.get_stat_effect()&EFF_ALL != NULL {
        sa.effect |= SAEFFECT_ATK_ALL_ENEMIES;
    }
    if eff.get_stat_effect()&EFF_ENEMY != NULL {
        sa.effect |= SAEFFECT_STATCHANGE_ON_ENEMY;
    }
    if eff.get_stat_effect()&EFF_ATK != NULL {
        sa.atk_buff += boost;
    }
    if eff.get_stat_effect()&EFF_DEF != NULL {
        sa.def_buff += boost;
    }
}

/// Call this function with the super attack lowercased!
#[wasm_bindgen]
pub fn parse_super_attack(sa_eff : &str) -> SaInfo
{
    let mut sa = SaInfo::default();
    let mut i : usize = 0;

    let sa_match = get_sa_match(sa_eff).expect("Failed to find match in super attack").as_str();
    sa.modifier = get_sa_modifier(sa_match).expect("Failed to retrieve super attack modifier");

    // Remove the sa modifier
    let mut s = SA_RE.replace(sa_eff, "").to_string();

    while tokenizer::has_more_tokens(&s)
    {
        let token = tokenizer::get_next_token(&mut s, false).expect("Could not retrieve next token");
        if tokenizer::is_skippable_token(&token) 
        {
            tokenizer::skip_token(&mut s);
            continue;
        }
        if token.1 == tokenizer::Token::Keyword 
        {
            // Check if it raises or lowers atk/def of enemy or self
            let eff_opt = effectparser::raises_or_lowers_stat(&mut s, true);
            if eff_opt.is_some() {
                let eff = eff_opt.expect("Failed to retrieve stat effect");
                get_sa_stat_change_eff(eff, &mut sa);
            }
        }
    }
    return sa;
}
