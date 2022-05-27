fn main() {
    let s = String::from("book");
    println!("I have one {} and you have two {}", s, pluralize(s.clone()));
}
fn pluralize(word: String) -> String {
    word + "s"
}
