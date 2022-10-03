use std::default::Default;

use crate::{tokenizer, sa::{self, SaInfo}};
use lazy_static::lazy_static;
use regex::Regex;

pub const NULL : u32 = 0x0;
pub const EFF_ATK : u32 = 0x1;
pub const EFF_DEF : u32 = 0x2;
pub const EFF_RAISES : u32 = 0x4;
pub const EFF_LOWERS : u32 = 0x8;
pub const EFF_GREATLY : u32 = 0x10;
pub const EFF_ENEMY : u32 = 0x20;
pub const EFF_ALL : u32 = 0x40;

pub struct Effect
{
    stat_eff : u32,
    stat_eff_turn_count : u32
}

fn get_stat_effect(s : &str) -> Option<u32>
{
    let mut eff : u32 = NULL;

    if s.contains("raises") || s.contains("increases") {
        eff |= EFF_RAISES;
    } else if s.contains("lowers") || s.contains("decreases") {
        eff |= EFF_LOWERS;
    } 

    if s.contains("greatly") {
        eff |= EFF_GREATLY;
    }

    if s.contains("enemy") || s.contains("enemies") {
        eff |= EFF_ENEMY;
    }

    if s.contains("all") {
        eff |= EFF_ALL;
    }

    if eff == NULL {
        return None;
    }

    return Some(eff);
}

pub fn raises_or_lowers_stat(s : &mut String, advance : bool) -> Option<Effect>
{
    lazy_static! {
        static ref RE : Regex = Regex::new(r"^((?:greatly )?(?:lowers|raises|increases|decreases) (?:all )?(?:(?:enemy|enemies) )?(?:atk|def))")
                                .expect("Failed to compile regex");
    }

    let re_match = RE.find(&s)?.as_str();
    let stat_eff = get_stat_effect(re_match).expect("Unable to retrieve any stat effect");

    if advance {
        *s = RE.replace(s, "").to_string();

    }
    let stat_eff_turn_count = 0;

    return Some(Effect{stat_eff, stat_eff_turn_count});
    
}