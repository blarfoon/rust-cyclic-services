pub struct Service2 {
    services: *const super::Services,
}

impl Service2 {
    pub fn new(services: *const super::Services) -> Service2 {
        Service2 { services }
    }

    pub fn call_service_1(&self) {
        unsafe { self.services.read().service1.do_something(3) }
    }
}
