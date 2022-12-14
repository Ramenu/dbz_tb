

use regex::Regex;
use lazy_static::lazy_static;
use std::{fmt, str::FromStr, mem};
use wasm_bindgen::prelude::*;

use crate::{traits, flags};

#[derive(Copy, Clone, PartialEq)]
pub enum Token 
{
    Op,
    Identifier,
    Keyword,
    Number,
    Null
}

#[derive(PartialEq, Copy, Clone)]
pub enum TokenKeywordType
{
    Generic,
    Stat,
    Type,
    Conditional
}

#[derive(PartialEq, Copy, Clone)]
pub enum ConditionalTokenType
{
    Generic,
    Comparator
}

#[derive(PartialEq, Copy, Clone)]
pub enum TokenOpType
{
    Generic,
    Modifier,
    Percentage
}

/* 
#[cfg(debug_assertions)]
impl fmt::Display for Token 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match mem::discriminant(&self) {
            mem::discriminant(&Token::Op(TokenOpType::Generic)) => "Binary Operator",
            _ => "Identifier"
        };
        write!(f, "{}", s)
    }
}*/

pub const fn invalid_token(token : Token) -> &'static str
{
    if token as u32 == Token::Op as u32 {
        return "Token is not classified as an 'OPERATOR'";
    } else if token as u32 == Token::Keyword as u32 {
        return "Token is not classified as a 'KEYWORD'";
    } else if token as u32 == Token::Identifier as u32 {
        return "Token is not classified as a 'IDENTIFIER'";
    } else if token as u32 == Token::Number as u32 {
        return "Token is not classified as a 'NUMBER'";
    }
    return "Token is not classified as 'NULL'";
}

/// Returns true if the token can be skipped (i.e., is of
/// little importance to the semantics of the sentence).
pub fn is_skippable_token(token : &(String, Token)) -> bool
{
    if token.1 != Token::Keyword && token.1 != Token::Op {
        return false;
    }
    return match token.0.as_str() {
        ";"|"and"|"," => true,
        _ => false
    };
}

/// Uses the regex 'r' to replace the match with
/// empty text. It also trims the leading whitespace.
pub fn advance_until(s : &str, r : &Regex) -> String
{
    let mut n = r.replace(s, "").to_string();
    trim_leading_whitespace(&mut n);
    return n;
}

/// Returns true if 's' is a number.
/// Note that this does not count decimal
/// numbers.
#[inline]
pub fn is_number(s : &str) -> bool {
    return s.parse::<i32>().is_ok();
}

#[inline]
pub fn get_token_as_num<T : FromStr>(s : &str) -> Result<T, T::Err> {
    return s.parse::<T>();
}

pub fn get_conditional_token_type(token : (&str, TokenKeywordType)) -> Option<ConditionalTokenType>
{
    if token.1 != TokenKeywordType::Conditional {
        return None;
    }
    return Some(match token.0 {
        "is"|"equal"|"to"|"="|"less"|"greater"|"more"|">"|"<"|"above"|"below" => ConditionalTokenType::Comparator,
        _ => ConditionalTokenType::Generic 
    });
}

/// Returns the category of the token. Requires that
/// 's' is a keyword token.
pub fn get_token_keyword_category(token : &(String, Token)) -> Option<TokenKeywordType>
{
    if token.1 != Token::Keyword {
        return None;
    }
    return Some(match token.0.as_str() {
        "atk"|"def"|"hp"|"ki" => TokenKeywordType::Stat,
        "str"|"phy"|"int"|"teq"|"agl" => TokenKeywordType::Type,
        "if"|"when"|"is"|"or"|"and"|"above"|"below" => TokenKeywordType::Conditional,
        _ => TokenKeywordType::Generic
    });
}

pub fn get_token_operator_category(token : &(String, Token)) -> Option<TokenOpType>
{
    if token.1 != Token::Op {
        return None;
    }
    return Some(match token.0.as_str() {
        "+"|"-" => TokenOpType::Modifier,
        "%" => TokenOpType::Percentage,
        _ => TokenOpType::Generic
    });
}

/// Simply advances 's' to get the next
/// token.
#[inline]
pub fn skip_token(s : &mut String) {
    get_next_token(s, true);
}

/// Converts the token string to a condition flag. This can work with any
/// type of token, but since it only examines one token, it cannot infer multiple
/// conditions. So only the following conditions can be returned:
/// * ConditionFlag::IF_EQUAL
/// * ConditionFlag::IF_ABOVE
/// * ConditionFlag::IF_BELOW
pub fn convert_token_str_to_comparsion_flag(s : &str) -> Option<flags::ConditionFlag> {
    return match s {
        "is"|"equal"|"=" => Some(flags::ConditionFlag::IF_EQUAL),
        "less"|"<"|"below" => Some(flags::ConditionFlag::IF_BELOW),
        "greater"|"more"|">"|"above" => Some(flags::ConditionFlag::IF_ABOVE),
        _ => None
    };
}


/// Returns the type of the token given a 
/// string argument.
pub fn get_token_type(s : &str) -> Token 
{
    let token: Token = match s {
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
        "etc" | "at" | "ultra-rare" | "deadly" => return Token::Keyword,

        // Note '/' can be used for OR options like 'Enemies/Allies' ATK +10%' 
        "+" | "-" | "*" | "/" | ";" | "&" | ">" | "=" | "<" | "\"" | "%" | "," => return Token::Op,
        _ => Token::Identifier
    };
    if s.parse::<i32>().is_ok() {
        return Token::Number;
    }

    return token;
}

#[inline]
/// Returns true if the string 's' contains any
/// more tokens.
pub fn has_more_tokens(s : &str) -> bool {
    return !s.trim().is_empty();
}

/// Returns the number of tokens contained in
/// the string 's'.
pub fn get_number_of_tokens(s : &str) -> usize
{
    let mut n : usize = 0;
    let mut s_cpy = String::from(s);

    while has_more_tokens(&s_cpy) {
        n += 1;
        get_next_token(&mut s_cpy, true);
    }
    return n;
}

/// Truncates the whitespace at the start of
/// the string.
fn trim_leading_whitespace(s : &mut String)
{
    while s.starts_with(' ') {
        s.remove(0);
    }
}

/// Returns an optional tuple consisting of a string and a token.
/// 's' is only modified iff 'advance' holds true. That is, the token
/// will be removed from the string. If a token cannot be found in a string
/// then None will be returned.
pub fn get_next_token(s : &mut String, advance : bool) -> Option<(String, Token)>
{
    lazy_static! {
        // Beware this regex is far from perfect at the moment
        static ref RE : Regex = Regex::new(r"([^\s\W]*[^\W][-']?\w*|[^\w\s])").expect("Failed to compile regex"); 
    }
    if has_more_tokens(&s) {
        if RE.is_match(s) {
            let found = RE.find(&s).expect("Unable to find match in string").as_str().to_string();
            let token = get_token_type(&found);
            if advance {
                *s = RE.replace(s, "").to_string();
                trim_leading_whitespace(s);
            }
            return Some((found, token));
        }
    }
    return None;
}

/// Returns a vector of the next 'n' tokens from the string 's'.
pub fn get_n_tokens(s : &mut String, n : u32, advance : bool) -> Vec<(String, Token)>
{
    let mut s_cpy = s.to_owned();
    let mut tokens : Vec<(String, Token)> = Vec::new();
    for _ in 0..n {
        let token = get_next_token(&mut s_cpy, true).expect("Failed to retrieve next token");
        tokens.push(token);
    }
    if advance {
        *s = s_cpy;
    }
    return tokens;
}
