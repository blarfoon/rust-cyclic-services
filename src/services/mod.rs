use std::{cell::UnsafeCell, mem::MaybeUninit, ptr, sync::Arc};

mod service1;
mod service2;

pub struct Services {
    pub service1: service1::Service1,
    pub service2: service2::Service2,
}

impl Services {
    pub fn new() -> Services {
        let raw_ptr = ptr::null::<Services>();

        let service1 = service1::Service1::new(raw_ptr);
        let service2 = service2::Service2::new(raw_ptr);

        Services { service1, service2 }
    }
}
