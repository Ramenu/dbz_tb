mod tokenizer;
mod sa;
mod tester;
mod effectparser;
fn main() 
{
    let s = "and causes immense damage to enemy".to_string();
    tester::test_sa_retrieval();
}