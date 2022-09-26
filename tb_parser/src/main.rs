mod tokenizer;
mod tester;

fn main() {
    const token : &str = "bob";
    tester::test_tokenizer(token);
}
