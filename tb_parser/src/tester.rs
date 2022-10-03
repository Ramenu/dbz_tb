use crate::{tokenizer, sa};

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

#[cfg(debug_assertions)]
pub fn test_sa_retrieval()
{
    let super_atks : [&str; 8] = [
        "causes low damage to enemy",
        "causes damage to enemy",
        "causes huge damage to enemy",
        "causes mass damage to all enemies",
        "causes extreme damage",
        "causes extreme damage to enemy",
        "causes supreme damage",
        "causes supreme damage to enemy"
    ];

    for s in super_atks 
    {
        let sa_match = sa::get_sa_match(&s).expect("Failed to find match in super attack").as_str();
        let modifier = sa::get_sa_modifier(sa_match).expect("Failed to retrieve super attack modifier");
        println!("Super attack is: {}\nModifier is: {}", sa_match, modifier);
    }

}