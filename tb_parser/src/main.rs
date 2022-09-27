mod tokenizer;
fn main() 
{
    let mut s = String::from("ATK and DEF +120% at start of turn; Ki +2 in addition per  Ki Sphere obtained; perform Potara fusion when conditions are met");
    while tokenizer::has_more_tokens(&s) {
        tokenizer::get_next_token(&mut s);
    }
}