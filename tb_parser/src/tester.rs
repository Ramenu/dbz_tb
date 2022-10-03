use crate::{tokenizer, sa};
use std::arch::asm;

/// Prints the token type for 's'.
#[cfg(debug_assertions)]
pub fn test_tokenizer(s : &String) {
    let result = tokenizer::get_token(s);
    match result {
        tokenizer::Token::Op => println!("Token is a binary operator!"),
        tokenizer::Token::Identifier => println!("Token is a identifier!"),
        tokenizer::Token::Keyword => println!("Token is a keyword!"),
        tokenizer::Token::Number => println!("Token is a number!"),
        tokenizer::Token::Null => println!("Token is null")
    };
}

/// Tests the retrieval of tokens from 's' by printing
/// each token. If anything wrong is found, fix it. It
/// also checks if get_number_of_tokens retrieves the actual
/// number of tokens in the string. Will panic if not.
#[cfg(debug_assertions)]
pub fn test_token_retrieval(s : &String)
{
    let mut ms = s.to_owned();
    let mut i : usize = 0;
    let token_count = tokenizer::get_number_of_tokens(&s);
    while tokenizer::has_more_tokens(&ms) {
        i += 1;
        let token = tokenizer::get_next_token(&mut ms, true).expect("Failed to retrieve next token");
        println!("{}", token.0);
    }
    // Make sure that the number of tokens matches the while loop
    assert!(token_count == i);
}

/// Prints super attack and modifier matches. You can check
/// if anything is suspicious by checking the print logs. If something
/// is suspicious, go fix it.
#[cfg(debug_assertions)]
pub fn test_sa_retrieval()
{
    let super_atks : [&str; 9] = [
        "causes low damage",
        "causes damage",
        "causes huge damage",
        "causes mass damage",
        "causes extreme damage",
        "causes supreme damage",
        "causes immense damage",
        "causes colossal damage",
        "causes mega-colossal damage"
    ];

    // Round 1
    for s in super_atks 
    {
        let sa_match = sa::get_sa_match(s).expect("Failed to find match in super attack").as_str();
        let modifier = sa::get_sa_modifier(sa_match).expect("Failed to retrieve super attack modifier");
        println!("\nTest string is: {}\n-----------\nSuper attack is: {}\nModifier is: {}", s, sa_match, modifier);
    }

    let mut t = String::from(" to enemy");
    // Round 2
    for _ in 0..2 
    {
        for s in super_atks 
        {
            let s = String::from(s) + &t;
            let sa_match = sa::get_sa_match(&s).expect("Failed to find match in super attack").as_str();
            let modifier = sa::get_sa_modifier(sa_match).expect("Failed to retrieve super attack modifier");
            println!("\nTest string is: {}\n-----------\nSuper attack is: {}\nModifier is: {}", s, sa_match, modifier);
        }
        t = " to all enemies".to_string();
    }

}