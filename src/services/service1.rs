use std::sync::RwLock;

pub struct Service1 {
    some_data: RwLock<i32>,
}

impl Service1 {
    pub fn new() -> Service1 {
        Service1 {
            some_data: RwLock::new(0),
        }
    }

    pub fn do_something(&self, data: i32) {
        println!("Service1::do_something()");

        let mut some_data = self.some_data.write().unwrap();
        *some_data = data;
    }
}
