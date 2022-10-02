use wasm_bindgen::prelude::*;
use regex::Regex;
use lazy_static::lazy_static;
use crate::tokenizer;

pub const MAX_SA_LEVEL : i32 = 25;

lazy_static! {
    static ref SA_RE : Regex = Regex::new(r"causes (low|damage|huge|extreme|mass|supreme|immense|colossal|mega-colossal) d?a?m?a?g?e? ?to a?l?l? ?(enemy|enemies)").expect("Failed to compile regex");
}

// Note that modifiers can be inferred if a '%' token is encountered, so take the appropriate action 
// to add or multiply based off of that
#[wasm_bindgen]
#[derive(Default)]
pub struct SaInfo
{
    modifier : Modifier,
    effect : SaEffect,
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

#[derive(Clone, Copy, Default)]
pub enum SaEffect
{
    #[default] Null = 0x0,
    Stun = 0x1,
    Seal = 0x2,
    AtkAllEnemies = 0x4
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
        "low" => Some(Modifier::Low),
        "damage" => Some(Modifier::Damage),
        "huge" => Some(Modifier::HugeDestructive),
        "extreme"|"mass" => Some(Modifier::ExtremeMass),
        "supreme" => Some(Modifier::Supreme),
        "immense" => Some(Modifier::Immense),
        "colossal" => Some(Modifier::Colossal),
        "mega-colossal" => Some(Modifier::MegaColossal),
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
    };
}

/// Returns the updated attack stat with the super attack buff. This
/// function should only be called at the very end and if the unit is
/// performing a super attack. 
pub fn get_sa_atk_stat(atk : f32, modifier_dmg : f32, sa_level : i32) -> f32
{
    debug_assert!(sa_level <= MAX_SA_LEVEL);
    return atk * modifier_dmg * sa_level as f32;
}

#[wasm_bindgen]
/// Call this function with the super attack lowercased!
pub fn parse_super_attack(sa_eff : &str) -> SaInfo
{
    let mut sa = SaInfo::default();
    let mut i : usize = 0;

    let sa_match = get_sa_match(sa_eff).expect("Failed to find match in super attack").as_str();
    sa.modifier = get_sa_modifier(sa_match).expect("Failed to retrieve super attack modifier");

    // Remove the sa modifier
    let mut s = SA_RE.replace(sa_eff, "").to_string();
    let no_of_tokens = tokenizer::get_number_of_tokens(&s);

    for _ in 0..no_of_tokens
    {
        let token = tokenizer::get_next_token(&mut s, false).expect("Could not retrieve next token");
        if tokenizer::is_skippable_token(&token) {
            tokenizer::skip_token(&mut s);
            continue;
        }
        if token.1 == tokenizer::Token::Keyword {

        }
    }
    return sa;
}
