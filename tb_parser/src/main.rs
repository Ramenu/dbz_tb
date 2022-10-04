mod tokenizer;
mod effectparser;
mod sa;
mod tester;
mod modifier;

fn main() 
{
    tester::test_super_attack_parsing(false);
}