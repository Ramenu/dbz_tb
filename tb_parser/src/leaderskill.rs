use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct LeaderSkillInfo
{
    s : u32
}

#[wasm_bindgen]
pub fn parse_leader_skill(leader_skill : &str) -> LeaderSkillInfo
{
    return LeaderSkillInfo { s: 1 };
}