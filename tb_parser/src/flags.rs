use {bitflags::bitflags};
use wasm_bindgen::prelude::wasm_bindgen;

pub const INC_OR_DEC_MODIFIER_PERCENTAGE : f32 = 1.3;
pub const GREATLY_INC_OR_DEC_MODIFIER_PERCENTAGE : f32 = 1.5;


bitflags! 
{
    #[wasm_bindgen]
    #[derive(Default)]
    pub struct EffectFlag : u32
    {
        const NONE = 0x0;
        const STUN = 0x1;
        const SEAL = 0x2;
        const ATK_ALL_ENEMIES = 0x4;
        const ATK = 0x8;
        const DEF = 0x10;
        const RAISES = 0x20;
        const LOWERS = 0x40;
        const GREATLY = 0x80;
        const STAT_TARGET_ALL = 0x100; // This is not the same as 'EFFECT_ATK_ALL_ENEMIES' this applies to stun/seal, etc.
        const ENEMY = 0x200;
        const INC_OR_DEC_MODIFIER = 0x400;
    }

    #[wasm_bindgen]
    pub struct TypeFlag : u32
    {
        const NONE = 0x0;
        const SUPER = 0x1;
        const EXTREME = 0x2;
        const STR = 0x4;
        const AGL = 0x8;
        const TEQ = 0x10;
        const PHY = 0x20;
        const INT = 0x40;
    }

    #[wasm_bindgen]
    #[derive(Default)]
    pub struct StatFlag : u32
    {
        const NONE = 0x0;
        const ATK = 0x1;
        const DEF = 0x2;
        const HP = 0x4;
        const KI = 0x8;
    }

    #[wasm_bindgen]
    #[derive(Default)]
    pub struct ConditionFlag : u32
    {
        const NONE = 0x0;
        const IF_ABOVE = 0x1;
        const IF_EQUAL = 0x2;
        const IF_BELOW = 0x4;
        const TURNS = 0x8;
        const PERCENTAGE = 0x10;
    }

    #[wasm_bindgen]
    #[derive(Default)]
    pub struct OpModifierFlag : u32
    {
        const NONE = 0x0;
        const PLUS = 0x1;
        const MINUS = 0x2;
        const PERCENTAGE = 0x4;
    }
}

pub fn convert_str_to_op_modifier_flag(s : &str) -> OpModifierFlag
{
    return match s {
        "+" => OpModifierFlag::PLUS,
        "-" => OpModifierFlag::MINUS,
        "%" => OpModifierFlag::PERCENTAGE,
        _ => OpModifierFlag::NONE
    };
}

/// Returns the typeflag representation of the string 
/// 's'.
pub fn convert_str_to_type_flag(s : &str) -> TypeFlag 
{
    return match s {
        "super str" => TypeFlag::SUPER|TypeFlag::STR,
        "super agl" => TypeFlag::SUPER|TypeFlag::AGL,
        "super teq" => TypeFlag::SUPER|TypeFlag::TEQ,
        "super phy" => TypeFlag::SUPER|TypeFlag::PHY,
        "super int" => TypeFlag::SUPER|TypeFlag::INT,
        "super" => TypeFlag::SUPER,
        "extreme str" => TypeFlag::EXTREME|TypeFlag::STR,
        "extreme agl" => TypeFlag::EXTREME|TypeFlag::AGL,
        "extreme teq" => TypeFlag::EXTREME|TypeFlag::TEQ,
        "extreme phy" => TypeFlag::EXTREME|TypeFlag::PHY,
        "extreme int" => TypeFlag::EXTREME|TypeFlag::INT,
        "extreme" => TypeFlag::EXTREME,
        "str" => TypeFlag::STR,
        "agl" => TypeFlag::AGL,
        "teq" => TypeFlag::TEQ,
        "phy" => TypeFlag::PHY,
        "int" => TypeFlag::INT,
        _ => TypeFlag::NONE
    };
}

/// Returns a stat flag representation of the string
/// 's'.
pub fn convert_str_to_stat_flag(s : &str) -> StatFlag
{
    return match s {
        "atk" => StatFlag::ATK,
        "def" => StatFlag::DEF,
        "hp" => StatFlag::HP,
        "ki" => StatFlag::KI,
        _ => StatFlag::NONE
    };
}



