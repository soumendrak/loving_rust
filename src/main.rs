fn main(){
    let mut list = vec![1, 2, 3];
    for i in list.into_iter() {
        println!("i is: {}", i);
        list.push(i + 1);
    }
}