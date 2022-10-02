use crate::{sa::SaInfo, tokenizer};
use regex::Regex;
use lazy_static::lazy_static;

fn matches_sa(s : &str) -> bool
{
    return match s
    {
        "causes low damage to enemy" |
        "causes damage to enemy" |
        "causes huge damage to enemy" |
        "causes extreme damage to enemy" |
        "causes mass damage to all enemies" |
        "causes supreme damage to enemy" |
        "causes immense damage to enemy" |
        "causes colossal damage to enemy" |
        "causes mega-colossal damage to enemy" => true,
        _ => false
    };
}

pub fn parse_super_attack(s : &mut String) -> Option<SaInfo>
{
    let mut sa : SaInfo;
    let mut n_tokens = tokenizer::get_number_of_tokens(s);

    if n_tokens >= 5
    {
        let next_five_tokens = tokenizer::get_n_tokens(s, 5, false);
        if &next_token.0 == "causes" 
        {

        }
    }
    return None;
}