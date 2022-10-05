mod tokenizer;
mod effectparser;
mod sa;
mod tester;
mod effect;

fn main() 
{
    tester::test_get_stun_effect();
    //tester::test_get_seal_effect();
    //tester::test_super_attack_parsing(false);
}