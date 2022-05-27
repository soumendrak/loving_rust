fn main() {
    let dice1 = 1;
    let dice2 = 1;
    match (dice1, dice2) {
        (1, 1) => println!("1,1"),
        (_, 5) | (5, _) => {
            println!("anyone is");
            println!("5");
        }
        _ => println!("None matched"),
    }
}
