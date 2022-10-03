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

impl Effect
{
    pub fn get_stat_effect(&self) -> u32 {
        return self.stat_eff;
    }

    pub fn get_stat_effect_turn_count(&self) -> u32 {
        return self.stat_eff_turn_count;
    }
}

/// This is only used for debugging to check 
/// what the status effect flag means. You can
/// use this to check and make sure the status
/// effect flags are working properly.
#[cfg(debug_assertions)]
pub fn stat_effect_flag_meaning(stat_eff : u32)
{
    let mut s = String::new();
    if stat_eff&EFF_GREATLY != NULL {
        s += "/greatly"
    }
    if stat_eff&EFF_RAISES != NULL {
        s += "/raises";
    }
    if stat_eff&EFF_LOWERS != NULL {
        s += "/lowers";
    }
    if stat_eff&EFF_ALL != NULL {
        s += "/all";
    }
    if stat_eff&EFF_ENEMY != NULL {
        s += "/enemy";
    }
    if stat_eff&EFF_ATK != NULL {
        s += "/atk";
    }
    if stat_eff&EFF_DEF != NULL {
        s += "/def";
    }

    if s.is_empty() { // stat_eff == NULL
        assert!(stat_eff == NULL);
    }

    println!("Status effect: {}", s);
}

/// Returns the stat effect as a flag.
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

    if s.contains("atk") {
        eff |= EFF_ATK;
    }

    if s.contains("def") {
        eff |= EFF_DEF;
    }

    if eff == NULL {
        return None;
    }

    return Some(eff);
}

/// Returns the number of turns the effect will last for.
/// By default it is 0 (permanant). If the 'advance' flag is
/// set to true then it will also advance the string and remove
/// all of the tokens up until the turn count is reached.
pub fn get_eff_turn_count(s : &mut String, advance : bool) -> u32
{
    lazy_static! {
        static ref TURN_COUNT_RE : Regex = Regex::new(r"^ ?for (\d*) turns?").expect("Failed to compile regex");
    }

    let mut turn_count : u32 = 0;

    let capture = TURN_COUNT_RE.captures(s);
    if capture.is_some() 
    {
        turn_count = capture.expect("Failed to find match")
                            .get(1)
                            .expect("Failed to retrieve first capture group")
                            .as_str()
                            .parse::<u32>()
                            .expect("Failed to convert string to u32");
        
        if advance {
            *s = TURN_COUNT_RE.replace(&s, "").to_string();
        }
    }

    return turn_count;

}

/// Returns an optional effect. If no stat-changing effect is found in the
/// string 's' then None will be returned. It can also advance the string
/// up until the end of the stat-changing effect if the 'advance' flag is
/// set to true.
pub fn raises_or_lowers_stat(s : &mut String, advance : bool) -> Option<Effect>
{
    lazy_static! {
        static ref RE : Regex = Regex::new(r"^((?:greatly )?(?:lowers|raises|increases|decreases) (?:all )?(?:(?:enemy|enemies) )?(?:atk|def))")
                                .expect("Failed to compile regex");
    }

    let re_match = RE.find(&s)?.as_str();
    let stat_eff = get_stat_effect(re_match).expect("Unable to retrieve any stat effect");

    if advance 
    {
        *s = RE.replace(s, "").to_string();
        return Some(Effect{stat_eff: stat_eff, stat_eff_turn_count: get_eff_turn_count(s, true)});
    }

    let mut tmp = RE.replace(s, "").to_string();
    return Some(Effect{stat_eff: stat_eff, stat_eff_turn_count: get_eff_turn_count(&mut tmp, false)});
}