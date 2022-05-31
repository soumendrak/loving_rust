fn say(s: &str){
    println!("I say: {}", s);
}
fn main(){
    let a = String::from("hello");
    say(&a);
    println!("trying to access a again: {}", &a)
}
