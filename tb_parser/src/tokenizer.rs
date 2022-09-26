use crate::wasm_bindgen;

#[wasm_bindgen]
pub enum Token {
    BinOp,
    Identifier,
    Keyword,
    Number
}


pub fn is_number(s : &String) -> bool {
    return s.parse::<i32>().is_ok();
}

