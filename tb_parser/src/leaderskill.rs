use std::cell::RefCell;

use regex::Regex;
use wasm_bindgen::prelude::*;

use crate::{flags, tokenizer};

#[wasm_bindgen]
pub struct LeaderSkillInfo
{
    types_boosted : flags::TypeFlag,
    stats_boosted : flags::StatFlag
}

enum Call
{
    ParseLeaderSkillStatBoosts
}

pub fn parse_leader_skill_stat_boosts(leader_skill : &mut String, info : &mut LeaderSkillInfo, advance : bool) -> Option<()>
{
    lazy_static::lazy_static! {
        static ref TYPE_LEADER_SKILL_BOOST_RE : Regex = Regex::new(r"(agl|int|phy|str|teq)?,? ?(agl|int|phy|str|teq)? ?(?:&|and)? ?(agl|int|phy|str|teq) type (atk|def|hp|ki) \+ ?[0-9]+%?").expect("Failed to compile regex");
    }
    let captures = TYPE_LEADER_SKILL_BOOST_RE.captures(&leader_skill)?;
    for i in 0..captures.len() {
        let type_flag = flags::convert_str_to_type_flag(&captures[i]);
        info.types_boosted |= type_flag;
        if type_flag == flags::TypeFlag::NONE {
            info.stats_boosted |= flags::convert_str_to_stat_flag(&captures[i]);
        }
    }

    if advance {
        tokenizer::advance_until(&leader_skill, &TYPE_LEADER_SKILL_BOOST_RE);
    }
    return Some(());
}


/// Leader skill must be passed as lower cased.
#[wasm_bindgen]
pub fn parse_leader_skill(leader_skill : &str) -> LeaderSkillInfo
{
    let mut leader_skill_cpy = String::from(leader_skill);
    let mut info = LeaderSkillInfo { types_boosted: flags::TypeFlag::NONE, stats_boosted: flags::StatFlag::NONE};

    let call_types = [
        Call::ParseLeaderSkillStatBoosts
    ];

    // Since there are many different types of leader skills, its important to just create seperate functions for
    // each different type
    while tokenizer::has_more_tokens(&leader_skill_cpy) {
        for call in call_types.iter() {
            let result = match call {
                Call::ParseLeaderSkillStatBoosts => parse_leader_skill_stat_boosts(&mut leader_skill_cpy, &mut info, true)
            };
            if result.is_none() {
                break;
            }
        }
    }
    return info;
}