

use regex::Regex;
use lazy_static::lazy_static;
use std::fmt;
use std::collections::{hash_map, HashMap};
use wasm_bindgen::prelude::*;

pub enum Token 
{
    Op,
    Identifier,
    Keyword,
    Number,
    Null
}

pub const CONDITIONAL : u32 = 0x0;
pub const TYPE : u32 = 0x1;
pub const STAT : u32 = 0x2;
pub const OTHER : u32 = 0x4;
pub const PERCENT : u32 = 0x8;
pub const BUFF : u32 = 0x10; 
pub const EFFECT : u32 = 0x20;
pub const SKIP : u32 = 0x30;
pub const NUM : u32 = 0x40;
pub const NERF : u32 = 0x50;

#[cfg(debug_assertions)]
impl fmt::Display for Token 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self 
        {
            Token::Op => "Binary Operator",
            Token::Keyword => "Keyword",
            Token::Number => "Number",
            _ => "Identifier"
        };
        write!(f, "{}", s)
    }
}

#[inline]
pub fn is_number(s : &String) -> bool {
    return s.parse::<i32>().is_ok();
}

#[inline]
pub fn s_is_binary_op(s : &str) -> bool 
{
    return match s {
        "+" | "-" | "*" | "/" | ";" | "&" => true,
        _ => false
    };
}

#[inline]
pub fn c_is_binary_op(c : char) -> bool {
    return match c 
    {
        '+' | '-' | '*' | '/' | ';' | '&' => true,
        _ => false
    };
}

pub fn get_binary_op(s : &str) -> Option<char> 
{
    for (_, c) in s.chars().enumerate() 
    {
        if c_is_binary_op(c) {
            return Some(c);
        }
    }
    return None;
}

pub fn is_conditional_token(s : &str) -> bool {
    return true;
}


pub fn get_token(s : &str) -> Token 
{
    lazy_static! 
    {
        static ref KEYWORD_TO_CATEGORIES : HashMap<&'static str, u32> = HashMap::from([
            ("raises", OTHER),
            ("atk", STAT),
            ("def", STAT),
            ("hp", STAT),
            ("ki", STAT),
            ("guard", STAT),
            ("str", TYPE),
            ("phy", TYPE),
            ("int", TYPE),
            ("teq", TYPE),
            ("agl", TYPE),
            ("if", CONDITIONAL),
            ("after", CONDITIONAL),
            ("when", CONDITIONAL),
            ("or", CONDITIONAL),
            ("only", CONDITIONAL),
            ("upon", CONDITIONAL),
            ("met", CONDITIONAL|OTHER),
            ("and", CONDITIONAL|OTHER),
            ("at", CONDITIONAL|OTHER),
            ("for", CONDITIONAL|OTHER),
            ("whose", CONDITIONAL), // Not really conditional, but in dokkan's case yes 
            ("rare", PERCENT),
            ("medium", PERCENT),
            ("high", PERCENT),
            ("great", PERCENT),
            ("greatly", PERCENT),
            ("every", PERCENT),
            ("low", PERCENT), // Note that this is not specific to SA multipliers
            ("chance", PERCENT),
            ("huge", PERCENT), // Note that this is not specific to SA multipliers
            // Below are exclusive to SA modifiers
            ("damage", PERCENT),
            ("huge", PERCENT),
            ("destructive", PERCENT),
            ("extreme", PERCENT),
            ("mass", PERCENT),
            ("supreme", PERCENT),
            ("immense", PERCENT),
            ("colossal", PERCENT),
            ("mega-colossal", PERCENT),
            // End of exclusive to SA modifiers
            ("1st", OTHER),
            ("2nd", OTHER),
            ("3rd", OTHER),
            ("first", OTHER),
            ("second", OTHER),
            ("third", OTHER),
            ("increases", PERCENT|BUFF),
            ("decreases", PERCENT|NERF),
            ("increasing", BUFF),
            ("decreasing", NERF),
            ("raises", BUFF),
            ("decreases", NERF),
            ("stunning", EFFECT),
            ("sealing", EFFECT),
            ("stun", EFFECT),
            ("seal", EFFECT),
            ("a", SKIP),
            ("an", SKIP),
            ("facing", OTHER),
            ("performing", OTHER),
            ("launches", OTHER),
            ("temporaily", NUM),


        ]);

    }
    let token: Token = match s.to_lowercase().as_str()
    {
        "raises" | "atk" | "def" | "hp" | "category" |
        "type" | "and" | "or" | "str" | "agl" | "teq" |
        "phy" | "int" | "ki" | "start" | "end" | "of" |
        "turn" | "in" | "addition" | "per" | "conditions" |
        "are" | "met" | "when" | "below" | "is" | "causes" |
        "to" | "deliver" | "the" | "recover" | "meter" | "more" |
        "as" | "for" | "once" | "only" | "rare" | "chance" |
        "stun" | "seal" | "all" | "allies" | "super" | "attacks" |
        "attack" | "performing" | "a" | "may" | "being" | "1st" |
        "attacker" | "enemy's" | "allies'" | "facing" | "enemies'" |
        "dealt" | "reduce" | "reduces" | "with" | "enemy" | 
        "above" | "damage" | "low" | "high" | "very" | "first" |
        "stunning" | "sealing" | "by" | "reducing" | "that" |
        "lowers" | "increases" | "increase" | "decreases" | 
        "decrease" | "guard" | "received" | "activated" |
        "activate" | "increasing" | "decreasing" | "disable" |
        "enable" | "reduced" | "active" | "immune" | "negative" |
        "effects" | "effect" | "final" | "blow" | "next" | "2nd" |
        "3rd" | "second" | "third" | "enemies" | "greatly" | 
        "briefly" | "upon" | "attacked" | "delivered" | 
        "temporarily" | "row" | "defense" | "massively" | "each" |
        "up" | "plus" | "an" | "additional" | "sphere" | "obtained" |
        "medium" | "evade" | "evading" | "greater" | "stats" | 
        "boost" | "starting" | "from" | "battle" | "counter" |
        "countering" | "tremendous" | "power" | "class" | "including" |
        "effective" | "against" | "target" | "ultimate" | "within" | 
        "same" | "after" | "launching" | "character" | "receives" | 
        "revives" | "ally" | "attacking" | "KO'd" | "KO" |
        "character's" | "recovered" | "great" | "huge" | "times" |
        "absorbs" | "can" | "obtain" | "whose" | "name" | "includes" |
        "less" | "transforms" | "entering" | "transformation" | "bigger" |
        "smaller" | "becoming" | "team" | "stunned" | "sealed" | "foresees" |
        "foresee" | "whenever" | "fuses" | "sneezes" | "switches" |
        "awakens" | "existing" | "exchange" | "randomly" | "using" | 
        "guaranteed" | "hit" | "into" | "rages" | "time" | "rises" |
        "turns" | "exchanges" | "another" | "through"| "if" | "deactivates" |
        "other" | "than" | "excluded" | "included" | "certain" | "entrance" |
        "animation" | "guards" | "critical" | "not" | "single" | "targeted" |
        "their" | "every" | "perform" | "rest" | "it" | "ultra" | "belong" |
        "etc" | "at" => return Token::Keyword,

        // Note '/' can be used for OR options like 'Enemies/Allies' ATK +10%' 
        "+" | "-" | "*" | "/" | ";" | "&" | ">" | "=" | "<" | "\"" | "%" => return Token::Op,
        _ => Token::Identifier
    };
    if String::from(s).parse::<i32>().is_ok() {
        return Token::Number;
    }

    return token;
}

#[inline]
pub fn has_more_tokens(s : &str) -> bool {
    return !s.trim().is_empty();
}

pub fn get_next_token(s : &mut String, advance : bool) -> Option<(String, Token)>
{
    lazy_static! {
        // Beware this regex is far from perfect at the moment
        static ref RE : Regex = Regex::new(r"([^\s\W]*[^\W][-']?\w*|[^\w\s])").expect("Failed to compile regex"); 
    }

    if RE.is_match(s) 
    {
        let found = RE.find(&s).expect("Unable to find match in string").as_str().to_string();
        let token = get_token(&found);
        if advance {
            *s = RE.replace(s, "").to_string();
        }
        return Some((found, token));
    }
    return None;
}
