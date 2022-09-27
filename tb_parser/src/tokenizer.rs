

use regex::Regex;
use lazy_static::lazy_static;
pub enum Token 
{
    BinOp,
    Identifier,
    Keyword,
    Number,
    Null
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
    let token: Token = match s 
    {
        "Raises" | "ATK" | "DEF" | "HP" | "Category" |
        "Type" | "and" | "or" | "STR" | "AGL" | "TEQ" |
        "PHY" | "INT" | "Ki" => return Token::Keyword,

        "+" | "-" | "*" | "/" | ";" | "&" => return Token::BinOp,
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

pub fn get_next_token(s : &mut String) -> Option<(String, Token)>
{
    lazy_static! {
        static ref RE : Regex = Regex::new(r"([^\s\W]*[^\W]|[^\w\s])").expect("Failed to compile regex"); //  WHEN U WAKE UP: this works
    }

    if RE.is_match(s) 
    {
        let found = RE.find(&s).expect("Unable to find match in string").as_str().to_string();
        let token = get_token(&found);
        *s = RE.replace(s, "").to_string();
        return Some((found, token));
    }
    return None;
}
