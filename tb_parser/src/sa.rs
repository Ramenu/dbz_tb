


pub const MAX_SA_LEVEL : i32 = 25;

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

/// Returns an optional tuple consisting of the SA modifier
/// and the damage modifier. If 's' is not an appropriate
/// super attack modifier word then it will return None.
fn get_sa_modifier(s : &str) -> Option<(Modifier, f32)>
{
    const SA_MODIFIER_LOW : f32 = 1.3;
    const SA_MODIFIER_DAMAGE : f32 = 1.7;
    const SA_MODIFIER_HUGE_DESTRUCTIVE : f32 = 2.0;
    const SA_MODIFIER_EXTREME_MASS : f32 = 2.2;
    const SA_MODIFIER_SUPREME : f32 = 2.5;
    const SA_MODIFIER_IMMENSE : f32 = 2.8;
    const SA_MODIFIER_COLOSSAL : f32 = 3.0;
    const SA_MODIFIER_MEGACOLOSSAL : f32 = 3.5;
    return match s
    {
        "low" => Some((Modifier::Low, SA_MODIFIER_LOW)),
        "damage" => Some((Modifier::Damage, SA_MODIFIER_DAMAGE)),
        "huge" => Some((Modifier::HugeDestructive, SA_MODIFIER_HUGE_DESTRUCTIVE)),
        "extreme"|"mass" => Some((Modifier::ExtremeMass, SA_MODIFIER_EXTREME_MASS)),
        "supreme" => Some((Modifier::Supreme, SA_MODIFIER_SUPREME)),
        "immense" => Some((Modifier::Immense, SA_MODIFIER_IMMENSE)),
        "colossal" => Some((Modifier::Colossal, SA_MODIFIER_COLOSSAL)),
        "mega-colossal" => Some((Modifier::MegaColossal, SA_MODIFIER_MEGACOLOSSAL)),
        _ => None
    };
}

/// Returns the updated attack stat with the super attack buff. This
/// function should only be called at the very end and if the unit is
/// performing a super attack. 
fn get_sa_atk_stat(atk : f32, modifier_dmg : f32, sa_level : i32) -> f32
{
    debug_assert!(sa_level <= MAX_SA_LEVEL);
    return atk * modifier_dmg * sa_level as f32;
}