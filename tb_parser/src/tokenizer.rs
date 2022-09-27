

use regex::Regex;
use lazy_static::lazy_static;
use std::fmt;

pub enum Token 
{
    Op,
    Identifier,
    Keyword,
    Number,
    Null
}

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


pub fn get_token(s : &str) -> Token 
{
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
        static ref RE : Regex = Regex::new(r"([^\s\W]*[\.']?[^\W][\.']?|[^\w\s])").expect("Failed to compile regex"); 
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
