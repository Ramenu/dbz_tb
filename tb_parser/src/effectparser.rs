use core::num;
use std::{default::Default, error::Error, string::ParseError, mem};

use crate::{tokenizer, sa::{self, SaInfo}, flags::{self, EffectFlag, OpFlag}};
use lazy_static::lazy_static;
use regex::Regex;
use wasm_bindgen::prelude::wasm_bindgen;

pub const ULTRA_RARE_CHANCE_PERCENTAGE : u32 = 1;
pub const RARE_CHANCE_PERCENTAGE : u32 = 15;
pub const MAY_CHANCE_PERCENTAGE : u32 = 20;
pub const CHANCE_PERCENTAGE : u32 = 25;
pub const MEDIUM_CHANCE_PERCENTAGE : u32 = 30;
pub const HIGH_CHANCE_PERCENTAGE : u32 = 50;
pub const GREAT_CHANCE_PERCENTAGE : u32 = 70;


pub struct StatEffect
{
    stat_eff : EffectFlag,
    stat_eff_turn_count : u32,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Default)]
pub struct EffectCondition
{
    pub condition : flags::ConditionFlag,
    pub condition_num : f32
}

pub struct EffectChance
{
    pub eff_chance : u32,
    pub eff_turn_count : u32,
    pub eff : EffectFlag
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Effect
{
    pub stats : flags::StatFlag,
    pub condition : EffectCondition,
    pub modifier_num : u32
}

impl EffectChance
{
    pub fn get_eff_chance(&self) -> u32 {
        return self.eff_chance;
    }
    pub fn get_eff_turn_count(&self) -> u32 {
        return self.eff_turn_count;
    }
    pub fn get_eff(&self) -> EffectFlag {
        return self.eff;
    }
}

impl StatEffect
{
    pub fn get_stat_effect(&self) -> EffectFlag {
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
pub fn stat_effect_flag_meaning(stat_eff : EffectFlag)
{
    let mut s = String::new();
    if stat_eff&EffectFlag::GREATLY != EffectFlag::NONE {
        s += "/greatly"
    }
    if stat_eff&EffectFlag::RAISES != EffectFlag::NONE {
        s += "/raises";
    }
    if stat_eff&EffectFlag::LOWERS != EffectFlag::NONE {
        s += "/lowers";
    }
    if stat_eff&EffectFlag::ATK_ALL_ENEMIES != EffectFlag::NONE {
        s += "/all";
    }
    if stat_eff&EffectFlag::ENEMY != EffectFlag::NONE {
        s += "/enemy";
    }
    if stat_eff&EffectFlag::ATK != EffectFlag::NONE {
        s += "/atk";
    }
    if stat_eff&EffectFlag::DEF != EffectFlag::NONE {
        s += "/def";
    }

    if s.is_empty() { // stat_eff == EffectFlag::NONE
        assert!(stat_eff == EffectFlag::NONE);
    }

    println!("Status effect: {}", s);
}


/// Returns the stat effect as a flag.
fn get_stat_effect(s : &str) -> Option<EffectFlag>
{
    let mut eff : EffectFlag = EffectFlag::NONE;

    if s.contains("raises") || s.contains("increases") {
        eff |= EffectFlag::RAISES;
    } else if s.contains("lowers") || s.contains("decreases") {
        eff |= EffectFlag::LOWERS|EffectFlag::ENEMY;
    } 

    if s.contains("greatly") {
        eff |= EffectFlag::GREATLY;
    }

    if s.contains("enemy") || s.contains("enemies") {
        eff |= EffectFlag::ENEMY;
    }

    if s.contains("all") {
        eff |= EffectFlag::ATK_ALL_ENEMIES;
    }

    if s.contains("atk") {
        eff |= EffectFlag::ATK;
    }

    if s.contains("def") {
        eff |= EffectFlag::DEF;
    }

    if eff == EffectFlag::NONE {
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
        static ref TURN_COUNT_RE : Regex = Regex::new(r"^for (\d*) turns?").expect("Failed to compile regex");
    }

    let mut turn_count : u32 = 0;

    if tokenizer::has_more_tokens(s)
    {
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
    }

    return turn_count;

}

/// Returns an optional effect. If no stat-changing effect is found in the
/// string 's' then None will be returned. It can also advance the string
/// up until the end of the stat-changing effect if the 'advance' flag is
/// set to true. Note that it won't advance if there is no stat-changing effect.
pub fn raises_or_lowers_stat(s : &mut String, advance : bool) -> Option<StatEffect>
{
    lazy_static! {
        static ref RE : Regex = Regex::new(r"^((?:greatly )?(?:lowers|raises|increases|decreases) (?:all )?(?:(?:enemy|enemies|enemy's) )?(?:atk|def))")
                                .expect("Failed to compile regex");
    }

    let re_match = RE.find(&s)?.as_str();
    let stat_eff = get_stat_effect(re_match).expect("Unable to retrieve any stat effect");

    if advance {
        *s = RE.replace(s, "").to_string();
        return Some(StatEffect{stat_eff: stat_eff, stat_eff_turn_count: get_eff_turn_count(s, true)});
    }

    let mut tmp = RE.replace(s, "").to_string();
    return Some(StatEffect{stat_eff: stat_eff, stat_eff_turn_count: get_eff_turn_count(&mut tmp, false)});
}

/// Returns the percentage chance. Note that
/// this is not completely identical to Dokkan's
/// chance system (it's way too mixed up).
pub fn get_chance_percentage(s : &str) -> u32
{
    return match s {
        "ultra-rare chance" => ULTRA_RARE_CHANCE_PERCENTAGE, // devilman sa
        "rare chance" => RARE_CHANCE_PERCENTAGE,
        "may" => MAY_CHANCE_PERCENTAGE,
        "chance"|"with a chance" => CHANCE_PERCENTAGE,
        "medium chance" => MEDIUM_CHANCE_PERCENTAGE,
        "high chance" => HIGH_CHANCE_PERCENTAGE,
        "great chance" => GREAT_CHANCE_PERCENTAGE,
        _ => 0
    };
}

/// Returns 1 if the string matches
/// 'all enemies' literally. This is not
/// returned as a boolean because the return
/// value is meant to be used as a flag.
#[inline]
pub fn targets_all_enemies(s : &str) -> bool {
    return s == "all enemies" || s == "all enemies'";
}

/// Returns effectchance, will just be all 0's if
/// there is no stun chance. 
pub fn get_stun_effect(s : &mut String, advance : bool) -> EffectChance
{
    lazy_static! {
        static ref RE : Regex = Regex::new(r"^((?:ultra-rare|rare|great|medium|high|with a)? ?chance|may) (?:to|of)? ?(?:stunning|stun) (the (?:attacked)? ?enemy|all enemies)(?: within the same turn)?")
                                .expect("Failed to compile regex");
    }

    let capture = RE.captures(s);

    if capture.is_some() {
        let mut num_turns;
        let capture_match = capture.expect("Failed to find match");
        let chance = capture_match.get(1).expect("Failed to retrieve first capture group").as_str();
        let target = capture_match.get(2).expect("Failed to retrieve second capture group").as_str();

        let stun_chance = get_chance_percentage(chance);
        let targets_all = match targets_all_enemies(target) {
            true => EffectFlag::STUN|EffectFlag::STAT_TARGET_ALL,
            false => EffectFlag::NONE
        };

        if advance {
            *s = tokenizer::advance_until(s, &RE);
            num_turns = get_eff_turn_count(s, true);
        } else {
            let mut tmp = tokenizer::advance_until(s, &RE);
            num_turns = get_eff_turn_count(&mut tmp, false);
        }

        /* num_turns would be 0 if a turn count is not specified. So adding by one is necessary if the
           stun chance is more than 0%. */
        if num_turns == 0 && stun_chance > 0 {
            num_turns += 1;
        }

        return EffectChance { eff_chance: stun_chance, 
                              eff_turn_count: num_turns, 
                              eff: targets_all };

    }

    return EffectChance { eff_chance: 0, eff_turn_count: 0, eff: EffectFlag::NONE };
}


/// Returns effectchance, will just be all 0's if
/// there is no eeal chance. 
pub fn get_seal_effect(s : &mut String, advance : bool) -> EffectChance
{
    lazy_static! {
        static ref RE : Regex = Regex::new(r"^((?:ultra-rare|rare|great|medium|high|with a)? ?chance|may)? ?(?:to|of)? ?(?:seal|seals|sealing) (?:that)? ?(the attacked enemy's|all enemies'|enemy's)? ?super attacks?")
                                .expect("Failed to compile regex");
    }

    let capture = RE.captures(s);

    if capture.is_some() {
        let mut num_turns;
        let mut seal_chance : u32 = 100; // Seal chance is guaranteed to be 100% if no match is found
        let mut targets_all : EffectFlag = EffectFlag::NONE; // Guaranteed to hit one enemy only if no match is found
        let capture_match = capture.expect("Failed to find match");
        let chance = capture_match.get(1);
        let target = capture_match.get(2);

        if chance.is_some() {
            seal_chance = get_chance_percentage(chance.expect("Failed to get first capture").as_str());
        }
        if target.is_some() {
            targets_all = match targets_all_enemies(target.expect("Failed to get second capture").as_str()) {
                true => EffectFlag::SEAL|EffectFlag::STAT_TARGET_ALL,
                false => EffectFlag::NONE
            };
        }

        if advance {
            *s = tokenizer::advance_until(s, &RE);
            num_turns = get_eff_turn_count(s, true);
        } else {
            let mut tmp = tokenizer::advance_until(s, &RE);
            num_turns = get_eff_turn_count(&mut tmp, false);
        }

        /* num_turns would be 0 if a turn count is not specified. So adding by one is necessary if the
           stun chance is more than 0%. */
        if num_turns == 0 && seal_chance > 0 {
            num_turns += 1;
        }

        return EffectChance { eff_chance: seal_chance, 
                              eff_turn_count: num_turns, 
                              eff: targets_all };

    }

    return EffectChance { eff_chance: 0, eff_turn_count: 0, eff: EffectFlag::NONE };
}


/// Parses conditional statements.
fn parse_condition(s : &mut String) -> EffectCondition
{
    let mut previous_token = String::new();
    let mut effect_condition = EffectCondition::default();


    let num_of_tokens = tokenizer::get_number_of_tokens(&s);
    for _ in 0..num_of_tokens {
        let token = tokenizer::get_next_token(s, true).expect("Failed to retrieve next token");
        
        if token.1 == tokenizer::Token::Keyword(tokenizer::TokenKeywordType::Conditional) {
            let conditional_token_type = tokenizer::get_conditional_token_type(&token.0)
                                                                          .expect("Failed to retrieve conditional token type");
            
            if conditional_token_type == tokenizer::ConditionalTokenType::Comparator {                                                    
                effect_condition.condition |= tokenizer::convert_token_str_to_comparsion_flag(&token.0)
                                                        .expect("Failed to convert token to a comparsion flag");
            }
        }
        else if token.1 == tokenizer::Token::Number {
            effect_condition.condition_num = tokenizer::get_token_as_num(&token.0).expect("Failed to convert token to number");
        }
        previous_token = token.0;
    }

    return effect_condition;
}

/// Parses any stat buffs/nerfs. This also parses the
/// conditional statements for when the buffs or nerfs 
/// may apply.
fn parse_stats(s : &mut String, effect : &mut Effect) -> Option<()>
{
    let mut op_modifier : flags::OpFlag = flags::OpFlag::NONE;
    let num_of_tokens = tokenizer::get_number_of_tokens(&s);
    for _ in 0..num_of_tokens {
        let token = tokenizer::get_next_token(s, true)?;

        if token.1 == tokenizer::Token::Keyword(tokenizer::TokenKeywordType::Stat) {
                effect.stats |= flags::convert_str_to_stat_flag(&token.0);
        }
        else if token.1 == tokenizer::Token::Keyword(tokenizer::TokenKeywordType::Conditional) {
            parse_condition(s); 
        }
        else if mem::discriminant(&token.1) == mem::discriminant(&tokenizer::Token::Op(tokenizer::TokenOpType::Generic)) {
            op_modifier |= flags::convert_str_to_op_flag(&token.0);
        }
        else if token.1 == tokenizer::Token::Number {
            effect.modifier_num = tokenizer::get_token_as_num(s).expect("Failed to convert token to number");
        }
        if tokenizer::is_skippable_token(&token) {
            continue;
        }
    }

    return None;
}

pub fn parse_effect(mut s : String) -> Effect
{
    let mut effect = Effect::default();
    let num_of_tokens = tokenizer::get_number_of_tokens(&s);
    for _ in 0..num_of_tokens {
        let token = tokenizer::get_next_token(&mut s, false).expect("Failed to retrieve next token");
        if token.1 == tokenizer::Token::Keyword(tokenizer::TokenKeywordType::Stat) {
            parse_stats(&mut s, &mut effect).expect("This error should be impossible to see, should you encounter this please file a bug report");
        }
    }
    return effect;
}