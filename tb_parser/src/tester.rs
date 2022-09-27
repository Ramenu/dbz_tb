use crate::tokenizer;


pub fn test_tokenizer(s : &str) {
    let result = tokenizer::get_token(s);
    match result {
        tokenizer::Tokens::Op => println!("Token is a binary operator!"),
        tokenizer::Tokens::Identifier => println!("Token is a identifier!"),
        tokenizer::Tokens::Keyword => println!("Token is a keyword!"),
        tokenizer::Tokens::Number => println!("Token is a number!"),
    }
}