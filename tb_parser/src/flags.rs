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
        const EFFECT_NULL = 0x0;
        const EFFECT_STUN = 0x1;
        const EFFECT_SEAL = 0x2;
        const EFFECT_ATK_ALL_ENEMIES = 0x4;
        const EFFECT_ATK = 0x8;
        const EFFECT_DEF = 0x10;
        const EFFECT_RAISES = 0x20;
        const EFFECT_LOWERS = 0x40;
        const EFFECT_GREATLY = 0x80;
        const EFFECT_STAT_TARGET_ALL = 0x100; // This is not the same as 'EFFECT_ATK_ALL_ENEMIES' this applies to stun/seal, etc.
        const EFFECT_ENEMY = 0x200;
        const EFFECT_INC_OR_DEC_MODIFIER = 0x400;
    }

    #[wasm_bindgen]
    #[derive(Default)]
    pub struct TypeFlag : u32
    {
        const TYPE_NONE = 0x0;
        const TYPE_SUPER = 0x1;
        const TYPE_EXTREME = 0x2;
        const TYPE_STR = 0x4;
        const TYPE_AGL = 0x8;
        const TYPE_TEQ = 0x10;
        const TYPE_PHY = 0x20;
        const TYPE_INT = 0x40;
    }
}



