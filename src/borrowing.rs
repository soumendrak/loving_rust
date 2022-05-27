fn main() {
    let s = String::from("book");
    println!("I have one {} and you have two {}", s, pluralize(&s));
}
fn pluralize(word: &str) -> String {
    word.to_owned() + "s"
}
