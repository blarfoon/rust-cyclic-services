use super::ServicesInner;

pub struct Service2 {}

impl Service2 {
    pub fn new() -> Service2 {
        Service2 {}
    }
}

impl ServicesInner<'_, &Service2> {
    pub fn do_something(&self) {
        println!("Service2::do_something()");
    }

    pub fn call_service_1(&self) {
        self.services.service1.do_something(42);
    }
}
