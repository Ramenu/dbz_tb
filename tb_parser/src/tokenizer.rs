

use regex::Regex;
use lazy_static::lazy_static;
use std::fmt;
use wasm_bindgen::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub enum Token 
{
    Op,
    Identifier,
    Keyword,
    Number,
    Null
}

pub enum TokenKeywordType
{
    Stat,
    Type,
    Generic
}

lazy_static! {
    // Beware this regex is far from perfect at the moment
    static ref RE : Regex = Regex::new(r"([^\s\W]*[^\W][-']?\w*|[^\w\s])").expect("Failed to compile regex"); 
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

/// Returns true if the token can be skipped (i.e., is of
/// little importance to the semantics of the sentence).
pub fn is_skippable_token(token : &(String, Token)) -> bool
{
    if token.1 != Token::Keyword && token.1 != Token::Op {
        return false;
    }
    return match token.0.as_str() {
        ";"|"and" => true,
        _ => false
    };
}

/// Uses the regex 'r' to replace the match with
/// empty text. It also trims the leading whitespace.
pub fn advance_until(s : &String, r : &Regex) -> String
{
    let mut n = r.replace(s, "").to_string();
    trim_leading_whitespace(&mut n);
    return n;
}

/// Returns true if 's' is a number.
/// Note that this does not count decimal
/// numbers.
#[inline]
pub fn is_number(s : &String) -> bool {
    return s.parse::<i32>().is_ok();
}

/// Returns the category of the token. Requires that
/// 's' is a keyword token.
pub fn get_token_keyword_category(s : &str) -> TokenKeywordType
{
    return match s 
    {
        "atk"|"def"|"hp" => TokenKeywordType::Stat,
        "str"|"phy"|"int"|"teq"|"agl" => TokenKeywordType::Type,
        _ => TokenKeywordType::Generic
    };
}

/// Simply advances 's' to get the next
/// token.
#[inline]
pub fn skip_token(s : &mut String) {
    get_next_token(s, true);
}



/// Returns the type of the token given a 
/// string argument.
pub fn get_token(s : &String) -> Token 
{
    let token: Token = match s.as_str()
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
    if has_more_tokens(&s)
    {
        if RE.is_match(s) 
        {
            let found = RE.find(&s).expect("Unable to find match in string").as_str().to_string();
            let token = get_token(&found);
            if advance {
                *s = RE.replace(s, "").to_string();
                trim_leading_whitespace(s);
            }
            return Some((found, token));
        }
    }
    return None;
}

/// Returns an optional tuple consisting of a string and a vector of tokens.
/// Retrieves 'n' tokens from the string 's', and also advances the string 'n'
/// times if the advance flag is set to true. If n == 0 or the string does not
/// contain 'n' or more tokens, then None will be returned and 's' will not be 
/// modified at all.
pub fn get_n_tokens(s : &mut String, n : u32, advance : bool) -> Option<(String, Vec<Token>)>
{
    if n == 0 {
        return None;
    }
    let mut appended_tokens = String::new();
    let mut s_cpy = s.to_owned();
    let mut tokens : Vec<Token> = Vec::new();
    for _ in 0..n {
        let token = get_next_token(&mut s_cpy, true)?;
        appended_tokens.push_str(" ");
        appended_tokens += &token.0;
        tokens.push(token.1);
    }
    if advance {
        *s = s_cpy;
    }
    appended_tokens.remove(0); // Just a whitespace character so dont need to have it there
    return Some((appended_tokens, tokens));
}
