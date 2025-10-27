use std::rc::Rc;

fn main() {
    let shared_data = Rc::new(String::from("Olá mundo"));
    println!("Original: {}", shared_data);

    println!("---");

    let listener1 = Rc::clone(&shared_data);
    println!("listener1: {}", listener1);
}
