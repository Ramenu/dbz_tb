use crate::tokenizer;


pub fn test_tokenizer(s : &str) {
    let result = tokenizer::get_token(s);
    match result {
        tokenizer::Tokens::BinOp(s) => println!("Token is a binary operator!"),
        tokenizer::Tokens::Identifier(s) => println!("Token is a identifier!"),
        tokenizer::Tokens::Keyword(s) => println!("Token is a keyword!"),
        tokenizer::Tokens::Number(s) => println!("Token is a number!"),
        _ => println!("NULL, something went wrong!")
    }
}