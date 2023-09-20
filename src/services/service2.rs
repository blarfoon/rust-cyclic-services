use std::sync::Arc;

pub struct Service2<'a> {
    services: Arc<super::Services<'a>>,
}

impl<'a> Service2<'a> {
    pub fn new(services: &Arc<super::Services<'a>>) -> Service2<'a> {
        Service2 {
            services: Arc::clone(services),
        }
    }

    pub fn do_something(&self) {
        println!("Service2::do_something()");
    }

    pub fn call_service_1(&self) {
        self.services.service1.do_something(3);
    }
}
