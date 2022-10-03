mod tokenizer;
mod sa;
//mod effectparser;
fn main() 
{
    let mut s = String::from("ATK and DEF +120% at start of turn; Ki +2 in addition per  Ki Sphere obtained; perform Potara fusion when conditions are met");
    while tokenizer::has_more_tokens(&s) {
        let tokens = tokenizer::get_n_tokens(&mut s, 4, true).expect("Not enough tokens");
        println!("{}", tokens.0);
    }
}