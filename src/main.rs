use std::rc::Rc;

fn main() {
    let shared_data = Rc::new(String::from("Olá mundo"));
    println!("Original: {}[{:p}]", shared_data, shared_data.as_ptr());

    println!("---");

    let listener1 = Rc::clone(&shared_data);
    println!("listener1: {}[{:p}]", listener1, listener1.as_ptr());

    println!("---");

    {
        let listener2 = Rc::clone(&listener1);
        println!("listener2: {}[{:p}]", listener2, listener2.as_ptr());
    }

    println!("---");
}
