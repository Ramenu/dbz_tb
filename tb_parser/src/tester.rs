use crate::tokenizer;


pub fn test_tokenizer(s : &str) {
    let result = tokenizer::get_token(s);
    match result {
        tokenizer::Tokens::BinOp(_s) => println!("Token is a binary operator!"),
        tokenizer::Tokens::Identifier(_s) => println!("Token is a identifier!"),
        tokenizer::Tokens::Keyword(_s) => println!("Token is a keyword!"),
        tokenizer::Tokens::Number(_s) => println!("Token is a number!"),
    }
}