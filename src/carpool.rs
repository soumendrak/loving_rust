#[derive(Debug)]
struct CarPool {
    passengers: Vec<String>,
}
impl CarPool {
    fn pick_up(&mut self, name: String) {
        self.passengers.push(name);
    }
}
fn main(){
    let mut monday_car_pool = CarPool {
        passengers: vec![],
    };
    monday_car_pool.pick_up(String::from("Soumendra"));
    println!("Carpool state: {:?}", monday_car_pool);

    monday_car_pool.pick_up(String::from("Narendra"));
    println!("Carpool state: {:?}", monday_car_pool);
}
