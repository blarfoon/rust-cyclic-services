use std::sync::{Arc, RwLock};

pub struct Service1<'a> {
    services: &'a Arc<super::Services<'a>>,
    some_data: RwLock<i32>,
}

impl<'a> Service1<'a> {
    pub fn new(services: &'a Arc<super::Services<'a>>) -> Service1<'a> {
        Service1 {
            services,
            some_data: RwLock::new(0),
        }
    }

    pub fn do_something(&self, data: i32) {
        println!("Service1::do_something()");

        let mut some_data = self.some_data.write().unwrap();
        *some_data = data;
    }
}
