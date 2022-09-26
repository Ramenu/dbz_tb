mod utils;
mod tokenizer;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, tb-parser!");
}


#[wasm_bindgen]
pub fn get_token(s : &str) -> tokenizer::Token {
    let token: tokenizer::Token = match s {
        "Raises" | "ATK" | "DEF" | "HP" | "Category" |
        "Type" | "and" | "or" | "STR" | "AGL" | "TEQ" |
        "PHY" | "INT" | "Ki" => return tokenizer::Token::Keyword,

        "+" | "-" | "*" | "/" => return tokenizer::Token::BinOp,
        _ => tokenizer::Token::Identifier
    };

    let s_as_num = String::from(s).parse::<i32>();
    
    if s_as_num.is_ok() {
        return tokenizer::Token::Number;
    }

    return token;
}