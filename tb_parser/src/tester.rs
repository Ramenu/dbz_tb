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
    let super_atks : [&str; 10] = [
        "causes low damage",
        "causes damage",
        "causes huge damage",
        "causes mass damage",
        "causes extreme damage",
        "causes supreme damage",
        "causes immense damage",
        "causes colossal damage",
        "causes mega-colossal damage",
        "low damage"
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

/// Tests if the stat raising and lowering is working as it
/// should.
#[cfg(debug_assertions)]
pub fn test_raises_or_lowers_stat()
{
    use crate::effectparser::{raises_or_lowers_stat, stat_effect_flag_meaning};
 
    let status_effects : [&str; 10] = [
        "raises atk",
        "raises def",
        "lowers atk",
        "lowers def",
        "greatly raises atk",
        "greatly raises def",
        "greatly lowers atk",
        "greatly lowers def",
        "raises atk for 5 turns",
        "raises atk for 1 turn"
    ];

    for s in status_effects
    {
        let mut tmp = s.to_string();
        let stat = raises_or_lowers_stat(&mut tmp, false).expect("Test failed. L");

        stat_effect_flag_meaning(stat.get_stat_effect());
        println!("Duration: {}", stat.get_stat_effect_turn_count());
    }
}

/// Should be called only when:
/// test_sa_retrieval() PASSED
/// test_raises_or_lowers_stat() PASSED
/// if this works, then the other two probably
/// work as well.
#[cfg(debug_assertions)]
pub fn test_super_attack_parsing(extensive_test : bool)
{
    if extensive_test
    {
        test_sa_retrieval();
        test_raises_or_lowers_stat();
        test_get_stun_effect();
    }
    let super_atks : [&str; 9] = [
        "causes huge damage to enemy and lowers atk",
        "causes huge damage to enemy and lowers def",
        "huge damage and rare chance to stun the enemy",
        "causes huge damage and may stun the enemy",
        "extreme damage and rare chance to stun the enemy",
        "huge damage and high chance to stun the enemy for 1 turn",
        "extreme damage and may stun the enemy for 2 turns",
        "low damage and great chance to stun the enemy for 3 turns",
        "causes extreme damage; may stun the attacked enemy within the same turn"
    ];

    for s in super_atks
    {
        let sa = sa::parse_super_attack(s);
        println!("Super attack modifier: {}
Super attack effect: {}
ATK Buff: {}
DEF Buff: {}
Stun chance: {}
Stun lasts for: {} turns
-------------------", sa.get_modifier(), sa.get_effect(), sa.get_atk_buff(), sa.get_def_buff(), sa.get_stun_chance(), sa.get_turns_to_stunseal());
    }
}

#[cfg(debug_assertions)]
pub fn test_get_stun_effect()
{
    use crate::effectparser;

    let effects : [&str; 19] = [
        "rare chance to stun the enemy",
        "medium chance to stun the enemy",
        "great chance to stun the enemy",
        "chance to stun the enemy",
        "high chance to stun all enemies",
        "may stun the enemy",
        "may stun all enemies",
        "rare chance to stun all enemies",
        "great chance to stun all enemies",
        "medium chance to stun all enemies",
        "high chance to stun all enemies",
        "rare chance of stunning the attacked enemy",
        "chance of stunning the enemy",
        "may stun the attacked enemy",
        "with a chance to stun the enemy",
        "rare chance to stun the enemy for 3 turns",
        "high chance to stun all enemies for 5 turns",
        "chance to stun the attacked enemy for 4 turns",
        "medium chance to stun the enemy for 1 turn"
    ];

    for s in effects
    {
        let mut s = s.to_string();
        let stun_eff = effectparser::get_stun_effect(&mut s, false);
        println!("Effect: {}\nStun chance: {}%\nStun all enemies: {}\nNumber of turns: {}\n--------", 
                 s, stun_eff.get_eff_chance(), stun_eff.get_on_all_enemies(), stun_eff.get_eff_turn_count());
    }


}