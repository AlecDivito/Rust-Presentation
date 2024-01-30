use std::{rc::Rc, sync::Arc};

mod algebraic_data_types;
mod power_type_system;

struct Slice<'a> {
    slice: &'a mut [u8],
}

// fn loop_slice<'a>(slice: &mut test)

fn main() {
    let mut list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice = Slice {
        slice: &mut list[0..5],
    };

    let a = Arc::new(10);
    let b = a;

    std::thread::spawn(move || {
        println!("{}", b);
    });

    crate::power_type_system::main();
    crate::algebraic_data_types::main();
}
