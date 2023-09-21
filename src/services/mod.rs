use std::sync::Arc;

use self::{service1::Service1, service2::Service2};

mod service1;
mod service2;

macro_rules! register_service {
    ($service:ident: $type:path) => {
        pub(crate) fn $service(&self) -> ServicesInner<$type> {
            ServicesInner {
                inner: &self.$service,
                services: self,
            }
        }
    };
}

pub struct Services {
    service1: service1::Service1,
    service2: service2::Service2,
}

impl Services {
    pub fn new() -> Arc<Services> {
        Arc::new(Services {
            service1: service1::Service1::new(),
            service2: service2::Service2::new(),
        })
    }

    register_service!(service1: Service1);
    register_service!(service2: Service2);
}

pub struct ServicesInner<'a, T> {
    pub inner: &'a T,
    services: &'a Services,
}
