fn fizz_buzz(num: i32) {
    let fizz = if num % 15 == 0 {
        "FizzBuzz"
    } else if num % 3 == 0 {
        "Buzz"
    } else if num % 5 == 0 {
        "Fizz"
    } else {
        " "
    };
    println!("The number is: {}", fizz)
}

fn main() {
    fizz_buzz(10);
    fizz_buzz(12);
    fizz_buzz(13);
    fizz_buzz(15);
    fizz_buzz(3);
}
