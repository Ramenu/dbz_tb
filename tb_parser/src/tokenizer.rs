use regex::Regex;

pub enum Tokens<'a> {
    BinOp(&'a str),
    Identifier(&'a str),
    Keyword(&'a str),
    Number(i32),
    Null
}


pub fn is_number(s : &String) -> bool {
    return s.parse::<i32>().is_ok();
}

pub fn get_token(s : &str) -> Tokens {
    let token: Tokens = match s {
        "Raises" | "ATK" | "DEF" | "HP" | "Category" |
        "Type" | "and" | "or" | "STR" | "AGL" | "TEQ" |
        "PHY" | "INT" | "Ki" => return Tokens::Keyword(s),

        "+" | "-" | "*" | "/" => return Tokens::BinOp(s),
        _ => Tokens::Identifier(s)
    };

    let s_as_num = String::from(s).parse::<i32>();
    
    if s_as_num.is_ok() {
        return Tokens::Number(s_as_num.expect("Failed to convert string to integer"));
    }

    return token;
}