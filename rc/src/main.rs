use std::rc::Rc;
fn main() {
    let pointer = Rc::new(1);

    {
        let second_pointer = pointer.clone();
        println!("{}", *second_pointer);
    }

    print!("{}", *pointer);
}