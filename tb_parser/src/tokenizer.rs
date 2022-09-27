
use regex::Regex;
use lazy_static::lazy_static;
pub enum Token {
    BinOp,
    Identifier,
    Keyword,
    Number
}

#[inline]
pub fn is_number(s : &String) -> bool {
    return s.parse::<i32>().is_ok();
}

pub fn get_token(s : &str) -> Token {
    let token: Token = match s {
        "Raises" | "ATK" | "DEF" | "HP" | "Category" |
        "Type" | "and" | "or" | "STR" | "AGL" | "TEQ" |
        "PHY" | "INT" | "Ki" => return Token::Keyword,

        "+" | "-" | "*" | "/" => return Token::BinOp,
        _ => Token::Identifier
    };

    if String::from(s).parse::<i32>().is_ok() {
        return Token::Number;
    }

    return token;
}

#[inline]
pub fn has_more_tokens(s : &str) -> bool {
    return !s.trim().is_empty()
}

pub fn get_next_token(s : &mut String) -> &str {
    lazy_static! {
        static ref RE : Regex = Regex::new(r"\S*\s?").expect("Failed to compile regex");
    }

    if RE.is_match(s) {
        *s = RE.replace(s, "").to_string();
        println!("{}", s);
    }
    return " ";
}







