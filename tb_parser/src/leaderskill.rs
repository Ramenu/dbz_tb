use std::cell::RefCell;

use regex::Regex;
use wasm_bindgen::prelude::*;

use crate::{flags, tokenizer};

#[wasm_bindgen]
#[derive(Copy, Clone, Default)]
pub struct TypeBoost
{
    pub stats_boosted : flags::StatFlag,
    pub boost_amount : f32
}

pub const STR_INDEX : usize = 0;
pub const AGL_INDEX : usize = 1;
pub const TEQ_INDEX : usize = 2;
pub const INT_INDEX : usize = 3;
pub const PHY_INDEX : usize = 4;
pub const SUPER_INDEX : usize = 5;
pub const SUPER_STR_INDEX : usize = 6;
pub const SUPER_AGL_INDEX : usize = 7;
pub const SUPER_TEQ_INDEX : usize = 8;
pub const SUPER_INT_INDEX : usize = 9;
pub const SUPER_PHY_INDEX : usize = 10;
pub const EXTREME_INDEX : usize = 11;
pub const EXTREME_STR_INDEX : usize = 12;
pub const EXTREME_AGL_INDEX : usize = 13;
pub const EXTREME_TEQ_INDEX : usize = 14;
pub const EXTREME_INT_INDEX : usize = 15;
pub const EXTREME_PHY_INDEX : usize = 16;

#[wasm_bindgen]
#[derive(Default)]
pub struct LeaderSkillInfo
{
    types : [TypeBoost; 17]
}

impl LeaderSkillInfo
{
    pub fn get_types(&self) -> [TypeBoost; 17] {
        return self.types;
    }
}

enum Callback
{
    ParseLeaderSkillStatBoosts
}

/// Parses the leader skill, more specifically the types the leader skill boost applies to 
/// and the stats. 
pub fn parse_leader_skill_stat_boosts(leader_skill : &mut String, info : &mut [TypeBoost; 17], advance : bool) -> Option<()>
{
    lazy_static::lazy_static! {
        static ref TYPE_LEADER_SKILL_BOOST_RE : Regex = Regex::new(r"(agl|int|phy|str|teq)?,? ?(agl|int|phy|str|teq)?,? ?(?:&|and)? ?(agl|int|phy|str|teq) type (atk|def|hp|ki) \+ ?([0-9]+%?)")
                                                               .expect("Failed to compile regex");
    }
    let mut indexes : Vec<usize> = Vec::new();
    let captures = TYPE_LEADER_SKILL_BOOST_RE.captures(&leader_skill)?;
    for capture in captures.iter() {
        if capture.is_some() {
            let capture_str = capture.expect("Failed to retrieve capture group").as_str();
            let type_flag = flags::convert_str_to_type_flag(capture_str);

            if type_flag != flags::TypeFlag::NONE {
                let index = match type_flag {
                    flags::TypeFlag::STR => STR_INDEX,
                    flags::TypeFlag::AGL => AGL_INDEX,
                    flags::TypeFlag::TEQ => TEQ_INDEX,
                    flags::TypeFlag::INT => INT_INDEX,
                    flags::TypeFlag::PHY => PHY_INDEX,
                    flags::TypeFlag::SUPER => SUPER_INDEX,
                    flags::TypeFlag::SUPER|flags::TypeFlag::STR => SUPER_STR_INDEX,
                    flags::TypeFlag::SUPER|flags::TypeFlag::AGL => SUPER_AGL_INDEX,
                    flags::TypeFlag::SUPER|flags::TypeFlag::TEQ => SUPER_TEQ_INDEX,
                    flags::TypeFlag::SUPER|flags::TypeFlag::INT => SUPER_INT_INDEX,
                    flags::TypeFlag::SUPER|flags::TypeFlag::PHY => SUPER_PHY_INDEX,
                    flags::TypeFlag::EXTREME => EXTREME_INDEX,
                    flags::TypeFlag::EXTREME|flags::TypeFlag::STR => EXTREME_STR_INDEX,
                    flags::TypeFlag::EXTREME|flags::TypeFlag::AGL => EXTREME_AGL_INDEX,
                    flags::TypeFlag::EXTREME|flags::TypeFlag::TEQ => EXTREME_TEQ_INDEX,
                    flags::TypeFlag::EXTREME|flags::TypeFlag::INT => EXTREME_INT_INDEX,
                    flags::TypeFlag::EXTREME|flags::TypeFlag::PHY => EXTREME_PHY_INDEX,
                    _ => panic!("Invalid flag or missing combination found, if you see this error, file a bug report")
                };
                indexes.push(index);
                continue;
            }
            let stat = flags::convert_str_to_stat_flag(capture_str);

            // To be extra cautious and make sure its not a number
            if stat != flags::StatFlag::NONE {
                for i in 0..indexes.len() {
                    info[indexes[i]].stats_boosted |= stat;
                }
            }
        }
    } 

    let mut boost = captures.get(captures.len() - 1)
                                  .expect("Failed to retrieve stat boost capture")
                                  .as_str();
            
    if boost.contains("%") { // Check if it isnt a flat boost 
        let replaced_text = &boost.replace("%", "");
        boost = replaced_text;
        let boost_num = boost.parse::<f32>().expect("Failed to convert string to f32");
        for i in 0..indexes.len() {
            info[indexes[i]].boost_amount = boost_num / 100.0;
        }
    } else {
        let boost_num = boost.parse::<f32>().expect("Failed to convert string to f32");
        for i in 0..indexes.len() {
            info[indexes[i]].stats_boosted |= flags::StatFlag::FLAT_BOOST;
            info[indexes[i]].boost_amount = boost_num;
        }
    }

    if advance {
        *leader_skill = tokenizer::advance_until(&leader_skill, &TYPE_LEADER_SKILL_BOOST_RE);
    }
    return Some(());
}


/// Leader skill must be passed as lower cased.
#[wasm_bindgen]
pub fn parse_leader_skill(mut leader_skill : String) -> LeaderSkillInfo
{
    let mut info = LeaderSkillInfo::default();

    let callbacks = [
        Callback::ParseLeaderSkillStatBoosts
    ];

    // Since there are many different types of leader skills, its important to just create seperate functions for
    // each different type
    while tokenizer::has_more_tokens(&leader_skill) {
        let leader_skill_before = leader_skill.to_owned();
        for call in callbacks.iter() {
            let result = match call {
                Callback::ParseLeaderSkillStatBoosts => parse_leader_skill_stat_boosts(&mut leader_skill, &mut info.types, true)
            };
            if result.is_none() {
                break;
            }
        }
        if &leader_skill_before == &leader_skill {
            break;
        }
    }
    return info;
}