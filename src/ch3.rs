fn next_birthday(name: &str, age: i32) {
    let next_age = age + 1;
    println!("Hi {} on next birthday you will be {}", name, next_age);
}

fn main() {
    next_birthday("Soumendra", 30);
    next_birthday("Upali", 26);
}
