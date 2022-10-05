use crate::{tokenizer, sa};
use std::arch::asm;
use crate::effectparser::*;

#[cfg(debug_assertions)]
 const CHANCE_COMBINATIONS : [(&str, u32); 8] = [
    ("ultra-rare chance", ULTRA_RARE_CHANCE_PERCENTAGE),
    ("rare chance", RARE_CHANCE_PERCENTAGE),
    ("may", MAY_CHANCE_PERCENTAGE),
    ("chance", CHANCE_PERCENTAGE),
    ("with a chance", CHANCE_PERCENTAGE),
    ("medium chance", MEDIUM_CHANCE_PERCENTAGE),
    ("high chance", HIGH_CHANCE_PERCENTAGE),
    ("great chance", GREAT_CHANCE_PERCENTAGE)
];

#[cfg(debug_assertions)]
const N_TURNS : [(&str, u32); 3] = [
    ("", 1),
    (" for 1 turn", 1),
    (" for 33 turns", 33)
];

#[cfg(debug_assertions)]
fn pass(s : &str)
{
    println!("{} - PASSED ✓", s);
}

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
        test_get_seal_effect();
    }

    const SA_MODIFIERS : [(&str, sa::Modifier); 9] = [
        ("low damage", sa::Modifier::Low),
        ("damage", sa::Modifier::Damage),
        ("huge damage", sa::Modifier::HugeDestructive),
        ("extreme damage", sa::Modifier::ExtremeMass),
        ("mass damage", sa::Modifier::ExtremeMass),
        ("supreme damage", sa::Modifier::Supreme),
        ("immense damage", sa::Modifier::Immense),
        ("colossal damage", sa::Modifier::Colossal),
        ("mega-colossal damage", sa::Modifier::MegaColossal)
    ];
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
-------------------", sa.get_modifier(), sa.get_effect(), sa.get_atk_buff(), sa.get_def_buff(), sa.get_stun_chance(), sa.get_turns_to_stun());
    }
}

#[cfg(debug_assertions)]
pub fn test_get_stun_effect()
{

    const END_COMBINATIONS : [(&str, u32); 7] = [
        ("to stun the enemy", 0),
        ("to stun all enemies", 1),
        ("stun the enemy", 0),
        ("stun all enemies", 1),
        ("of stunning the attacked enemy", 0),
        ("to stun the attacked enemy", 0),
        ("stun the attacked enemy within the same turn", 0)
    ];

    for chance in CHANCE_COMBINATIONS
    {
        for end in END_COMBINATIONS
        {
            for turn_count in N_TURNS
            {
                let mut s = chance.0.to_string() + " " + &end.0.to_string() + &turn_count.0.to_string();
                let eff = get_stun_effect(&mut s, false);

                assert_eq!(chance.1, eff.eff_chance);
                assert_eq!(end.1, eff.on_all_enemies);
                assert_eq!(turn_count.1, eff.eff_turn_count);
            }
        }
    }

    pass("test_get_stun_effect()");

}

#[cfg(debug_assertions)]
pub fn test_get_seal_effect()
{
    let effects : [(&str, EffectChance); 10] = [
        ("rare chance to seal all enemies' super attack", EffectChance{eff_chance: RARE_CHANCE_PERCENTAGE, eff_turn_count: 1, on_all_enemies: 1}),
        ("may seal enemy's super attack", EffectChance{eff_chance: MAY_CHANCE_PERCENTAGE, eff_turn_count: 1, on_all_enemies: 0}),
        ("seals super attack", EffectChance{eff_chance: 100, eff_turn_count: 1, on_all_enemies: 0}),
        ("medium chance to seal super attack", EffectChance{eff_chance: MEDIUM_CHANCE_PERCENTAGE, eff_turn_count: 1, on_all_enemies: 0}),
        ("high chance to seal the attacked enemy's super attack", EffectChance{eff_chance: HIGH_CHANCE_PERCENTAGE, eff_turn_count: 1, on_all_enemies: 0}),
        ("high chance of sealing super attack", EffectChance{eff_chance: HIGH_CHANCE_PERCENTAGE, eff_turn_count: 1, on_all_enemies: 0}),
        ("medium chance of sealing super attack", EffectChance{eff_chance: MEDIUM_CHANCE_PERCENTAGE, eff_turn_count: 1, on_all_enemies: 0}),
        ("seals that enemy's super attack", EffectChance{eff_chance: 100, eff_turn_count: 1, on_all_enemies: 0}),
        ("high chance of sealing super attack for 3 turns", EffectChance{eff_chance: HIGH_CHANCE_PERCENTAGE, eff_turn_count: 3, on_all_enemies: 0}),
        ("empty string", EffectChance{eff_chance: 0, eff_turn_count: 0, on_all_enemies: 0})
    ];

    for eff in effects
    {
        let mut s = eff.0.to_string();
        let test_eff = get_seal_effect(&mut s, false);
        assert_eq!(eff.1.eff_chance, test_eff.eff_chance);
        assert_eq!(eff.1.eff_turn_count, test_eff.eff_turn_count);
        assert_eq!(eff.1.on_all_enemies, test_eff.on_all_enemies);
    }

    pass("test_get_seal_effect()");
}