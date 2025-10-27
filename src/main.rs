use std::rc::Rc;

fn main() {
    let shared_data = Rc::new(String::from("Olá mundo"));
    println!("Original: {}[{:p}]", shared_data, shared_data.as_ptr());

    println!("---");

    let listener1 = Rc::clone(&shared_data);
    println!("listener1: {}[{:p}]", listener1, listener1.as_ptr());
}
