use std::sync::{Arc};
use std::{thread, time};

fn main() {
    let pointer = Arc::new(5);

    let second_pointer = pointer.clone();
    thread::spawn(move || {
        println!("{}", *second_pointer);
    });

    thread::sleep(time::Duration::from_secs(1));

    print!("{}", *pointer);
}
