use std::{cell::UnsafeCell, mem::MaybeUninit, sync::Arc};

mod service1;
mod service2;

pub struct Services<'a> {
    pub service1: service1::Service1<'a>,
    pub service2: service2::Service2<'a>,
}

impl<'a> Services<'a> {
    pub fn new() -> Arc<Services<'a>> {
        let mut uninit_services: UnsafeCell<MaybeUninit<Arc<Services>>> =
            UnsafeCell::new(MaybeUninit::uninit());

        let arced_services = uninit_services.get_mut().as_ptr();

        let service1 = service1::Service1::new(unsafe { &*arced_services });
        let service2 = service2::Service2::new(unsafe { &*arced_services });

        let services = Arc::new(Services { service1, service2 });

        *uninit_services.get_mut() = MaybeUninit::new(services.clone());

        services
    }
}
