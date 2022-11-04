use crate::{tokenizer, sa};
use crate::effectparser::*;
use crate::flags::*;
use crate::leaderskill;
use colored::Colorize;

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
const TYPES : [(&str, u32); 85] = [
    ("phy", 0x20),
    ("phy and teq", 0x30),
    ("phy, teq, and str", 0x34),
    ("phy, teq, and int", 0x70),
    ("phy, teq, and agl", 0x38),
    ("phy and str", 0x24),
    ("phy, str, and teq", 0x34),
    ("phy, str, and int", 0x64),
    ("phy, str, and agl", 0x2c),
    ("phy and int", 0x60),
    ("phy, int, and teq", 0x70),
    ("phy, int, and str", 0x64),
    ("phy, int, and agl", 0x68),
    ("phy and agl", 0x28),
    ("phy, agl, and teq", 0x38),
    ("phy, agl, and str", 0x2c),
    ("phy, agl, and int", 0x68),
    ("teq", 0x10),
    ("teq and phy", 0x30),
    ("teq, phy, and str", 0x34),
    ("teq, phy, and int", 0x70),
    ("teq, phy, and agl", 0x38),
    ("teq and str", 0x14),
    ("teq, str, and phy", 0x34),
    ("teq, str, and int", 0x54),
    ("teq, str, and agl", 0x1c),
    ("teq and int", 0x50),
    ("teq, int, and phy", 0x70),
    ("teq, int, and str", 0x54),
    ("teq, int, and agl", 0x58),
    ("teq and agl", 0x18),
    ("teq, agl, and phy", 0x38),
    ("teq, agl, and str", 0x1c),
    ("teq, agl, and int", 0x58),
    ("str", 0x4),
    ("str and phy", 0x24),
    ("str, phy, and teq", 0x34),
    ("str, phy, and int", 0x64),
    ("str, phy, and agl", 0x2c),
    ("str and teq", 0x14),
    ("str, teq, and phy", 0x34),
    ("str, teq, and int", 0x54),
    ("str, teq, and agl", 0x1c),
    ("str and int", 0x44),
    ("str, int, and phy", 0x64),
    ("str, int, and teq", 0x54),
    ("str, int, and agl", 0x4c),
    ("str and agl", 0xc),
    ("str, agl, and phy", 0x2c),
    ("str, agl, and teq", 0x1c),
    ("str, agl, and int", 0x4c),
    ("int", 0x40),
    ("int and phy", 0x60),
    ("int, phy, and teq", 0x70),
    ("int, phy, and str", 0x64),
    ("int, phy, and agl", 0x68),
    ("int and teq", 0x50),
    ("int, teq, and phy", 0x70),
    ("int, teq, and str", 0x54),
    ("int, teq, and agl", 0x58),
    ("int and str", 0x44),
    ("int, str, and phy", 0x64),
    ("int, str, and teq", 0x54),
    ("int, str, and agl", 0x4c),
    ("int and agl", 0x48),
    ("int, agl, and phy", 0x68),
    ("int, agl, and teq", 0x58),
    ("int, agl, and str", 0x4c),
    ("agl", 0x8),
    ("agl and phy", 0x28),
    ("agl, phy, and teq", 0x38),
    ("agl, phy, and str", 0x2c),
    ("agl, phy, and int", 0x68),
    ("agl and teq", 0x18),
    ("agl, teq, and phy", 0x38),
    ("agl, teq, and str", 0x1c),
    ("agl, teq, and int", 0x58),
    ("agl and str", 0xc),
    ("agl, str, and phy", 0x2c),
    ("agl, str, and teq", 0x1c),
    ("agl, str, and int", 0x4c),
    ("agl and int", 0x48),
    ("agl, int, and phy", 0x68),
    ("agl, int, and teq", 0x58),
    ("agl, int, and str", 0x4c),

];

#[cfg(debug_assertions)]
const STATS : [(&str, StatFlag); 4] = [
    ("atk", StatFlag::ATK),
    ("def", StatFlag::DEF),
    ("hp", StatFlag::HP),
    ("ki", StatFlag::KI)
];

const NUMS : [&str; 7] = [
    "2",
    "0",
    "12",
    "3%",
    "55%",
    "5444",
    "5444%"
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
    println!("{} - {}", s, "PASSED ✓".green());
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
        _ => println!("Token is null")
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
    for s in super_atks {
        let sa_match = sa::get_sa_match(s).expect("Failed to find match in super attack").as_str();
        let modifier = sa::get_sa_modifier(sa_match).expect("Failed to retrieve super attack modifier");
        println!("\nTest string is: {}\n-----------\nSuper attack is: {}\nModifier is: {}", s, sa_match, modifier);
    }

    let mut t = String::from(" to enemy");
    // Round 2
    for _ in 0..2 {
        for s in super_atks {
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

    for s in status_effects {
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
    if extensive_test {
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

    const START_WORDS : [&str; 2] = [
        "",
        "causes "
    ];

    struct StatChange
    {
        pub atk_buff : f32,
        pub def_buff : f32
    }

    const END_WORDS : [(&str, StatChange); 9] = [
        ("", StatChange{atk_buff: 0.0, def_buff: 0.0}),
        (" and lowers atk", StatChange{atk_buff: -INC_OR_DEC_MODIFIER_PERCENTAGE, def_buff: 0.0}),
        (" and lowers def", StatChange{atk_buff: 0.0, def_buff: -INC_OR_DEC_MODIFIER_PERCENTAGE}),
        (" and greatly lowers atk", StatChange{atk_buff: -GREATLY_INC_OR_DEC_MODIFIER_PERCENTAGE, def_buff: 0.0}),
        (" and greatly lowers def", StatChange{atk_buff: 0.0, def_buff: -GREATLY_INC_OR_DEC_MODIFIER_PERCENTAGE}),

        (" and raises atk", StatChange{atk_buff: INC_OR_DEC_MODIFIER_PERCENTAGE, def_buff: 0.0}),
        (" and raises def", StatChange{atk_buff: 0.0, def_buff: INC_OR_DEC_MODIFIER_PERCENTAGE}),
        (" and greatly raises atk", StatChange{atk_buff: GREATLY_INC_OR_DEC_MODIFIER_PERCENTAGE, def_buff: 0.0}),
        (" and greatly raises def", StatChange{atk_buff: 0.0, def_buff: GREATLY_INC_OR_DEC_MODIFIER_PERCENTAGE})
    ];

    for start in START_WORDS {
        for modifier in SA_MODIFIERS {
            for end in END_WORDS {
                let s = start.to_string() + modifier.0 + end.0;
                let sa = sa::parse_super_attack(&s);

                assert_eq!(sa.get_modifier(), modifier.1);
                assert_eq!(sa.get_atk_buff(), end.1.atk_buff);
                assert_eq!(sa.get_def_buff(), end.1.def_buff);
            }
        }
    }

    pass("test_super_attack_parsing()");
}

#[cfg(debug_assertions)]
pub fn test_get_stun_effect()
{

    let end_combinations : [(&str, EffectFlag); 7] = [
        ("to stun the enemy", EffectFlag::NONE),
        ("to stun all enemies", EffectFlag::STUN|EffectFlag::STAT_TARGET_ALL),
        ("stun the enemy", EffectFlag::NONE),
        ("stun all enemies", EffectFlag::STUN|EffectFlag::STAT_TARGET_ALL),
        ("of stunning the attacked enemy", EffectFlag::NONE),
        ("to stun the attacked enemy", EffectFlag::NONE),
        ("stun the attacked enemy within the same turn", EffectFlag::NONE)
    ];

    for chance in CHANCE_COMBINATIONS {
        for end in end_combinations {
            for turn_count in N_TURNS {
                let mut s = chance.0.to_string() + " " + &end.0.to_string() + &turn_count.0.to_string();
                let eff = get_stun_effect(&mut s, false);

                assert_eq!(chance.1, eff.eff_chance);
                assert_eq!(end.1, eff.eff);
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
        ("rare chance to seal all enemies' super attack", EffectChance{eff_chance: RARE_CHANCE_PERCENTAGE, eff_turn_count: 1, eff: EffectFlag::SEAL|EffectFlag::STAT_TARGET_ALL}),
        ("may seal enemy's super attack", EffectChance{eff_chance: MAY_CHANCE_PERCENTAGE, eff_turn_count: 1, eff: EffectFlag::NONE}),
        ("seals super attack", EffectChance{eff_chance: 100, eff_turn_count: 1, eff: EffectFlag::NONE}),
        ("medium chance to seal super attack", EffectChance{eff_chance: MEDIUM_CHANCE_PERCENTAGE, eff_turn_count: 1, eff: EffectFlag::NONE}),
        ("high chance to seal the attacked enemy's super attack", EffectChance{eff_chance: HIGH_CHANCE_PERCENTAGE, eff_turn_count: 1, eff: EffectFlag::NONE}),
        ("high chance of sealing super attack", EffectChance{eff_chance: HIGH_CHANCE_PERCENTAGE, eff_turn_count: 1, eff: EffectFlag::NONE}),
        ("medium chance of sealing super attack", EffectChance{eff_chance: MEDIUM_CHANCE_PERCENTAGE, eff_turn_count: 1, eff: EffectFlag::NONE}),
        ("seals that enemy's super attack", EffectChance{eff_chance: 100, eff_turn_count: 1, eff: EffectFlag::NONE}),
        ("high chance of sealing super attack for 3 turns", EffectChance{eff_chance: HIGH_CHANCE_PERCENTAGE, eff_turn_count: 3, eff: EffectFlag::NONE}),
        ("empty string", EffectChance{eff_chance: 0, eff_turn_count: 0, eff: EffectFlag::NONE})
    ];

    for eff in effects {
        let mut s = eff.0.to_string();
        let test_eff = get_seal_effect(&mut s, false);
        assert_eq!(eff.1.eff_chance, test_eff.eff_chance);
        assert_eq!(eff.1.eff_turn_count, test_eff.eff_turn_count);
        assert_eq!(eff.1.eff, test_eff.eff);
    }

    pass("test_get_seal_effect()");
}

#[cfg(debug_assertions)]
pub fn test_leader_skill_parsing()
{
    // Round 1

    use crate::leaderskill::{TEQ_INDEX, STR_INDEX, AGL_INDEX, INT_INDEX, PHY_INDEX, EXTREME_TEQ_INDEX};

    let leader_skills = [
        ("teq type def +20%", StatFlag::DEF, 0.2, vec![TEQ_INDEX]),
        ("str type atk +2500", StatFlag::ATK|StatFlag::FLAT_BOOST, 2500.0, vec![STR_INDEX]),
        ("agl, int, and phy type atk +30%", StatFlag::ATK, 0.3, vec![AGL_INDEX, INT_INDEX, PHY_INDEX]),
        ("teq and str type ki +2", StatFlag::KI|StatFlag::FLAT_BOOST, 2.0, vec![TEQ_INDEX, STR_INDEX])
    ];

    for ls in leader_skills {
        let info = leaderskill::parse_leader_skill(ls.0.to_string());

        for i in 0..ls.3.len() {
            assert_eq!(info.get_types()[ls.3[i]].stats_boosted, ls.1);
            assert_eq!(info.get_types()[ls.3[i]].boost_amount, ls.2);
        }
    }

    pass("test_leader_skill_parsing()");
}