

const SA_MODIFIER_LOW : f32 = 1.3;
const SA_MODIFIER_DAMAGE : f32 = 1.7;
const SA_MODIFIER_HUGE_DESTRUCTIVE : f32 = 2.0;
const SA_MODIFIER_EXTREME_MASS : f32 = 2.2;
const SA_MODIFIER_SUPREME : f32 = 2.5;
const SA_MODIFIER_IMMENSE : f32 = 2.8;
const SA_MODIFIER_COLOSSAL : f32 = 3.0;
const SA_MODIFIER_MEGACOLOSSAL : f32 = 3.5;

pub const MAX_SA_LEVEL : u8 = 25;

enum Modifier 
{
    Low,
    Damage,
    HugeDestructive,
    ExtremeMass,
    Supreme,
    Immense,
    Colossal,
    MegaColossal
}

fn get_sa_modifier(s : &str) -> (Modifier, f32)
{
    return match s
    {
        "low" => (Modifier::Low, SA_MODIFIER_LOW),
        "damage" => (Modifier::Damage, SA_MODIFIER_DAMAGE),
        "huge" => (Modifier::HugeDestructive, SA_MODIFIER_HUGE_DESTRUCTIVE),
        "extreme"|"mass" => (Modifier::ExtremeMass, SA_MODIFIER_EXTREME_MASS),
        "supreme" => (Modifier::Supreme, SA_MODIFIER_SUPREME),
        "immense" => (Modifier::Immense, SA_MODIFIER_IMMENSE),
        "colossal" => (Modifier::Colossal, SA_MODIFIER_COLOSSAL),
        _ => (Modifier::MegaColossal, SA_MODIFIER_MEGACOLOSSAL)
    };
}

fn get_sa_atk_stat(atk : f32, modifier_dmg : f32, sa_level : u8) -> f32
{
    debug_assert!(sa_level <= MAX_SA_LEVEL);
    return atk * modifier_dmg * sa_level as f32;
}