use std::{cell::UnsafeCell, mem::MaybeUninit, sync::Arc};

use self::{service1::Service1, service2::Service2};

mod service1;
mod service2;

pub struct Services {
    pub service1: service1::Service1,
    pub service2: service2::Service2,
}

impl Services {
    pub fn new() -> Arc<Services> {
        let services = Arc::new(Services {
            service1: service1::Service1::new(),
            service2: service2::Service2::new(),
        });

        services
    }

    pub fn service1(&self) -> ServicesInner<&Service1> {
        ServicesInner {
            inner: &self.service1,
            services: self,
        }
    }

    pub fn service2(&self) -> ServicesInner<&Service2> {
        ServicesInner {
            inner: &self.service2,
            services: self,
        }
    }
}

pub struct ServicesInner<'a, T> {
    pub inner: T,
    services: &'a Services,
}
