use crate::{tokenizer, sa::{self, SaInfo}};
use lazy_static::lazy_static;
use regex::Regex;


pub fn raises_or_lowers_stat(s : &mut String) -> sa::SaInfo
{
    lazy_static! {
        static ref RE : Regex = Regex::new(r"^(\w* ?raises|lowers) (ATK|DEF)").expect("Failed to compile regex");
    }


}