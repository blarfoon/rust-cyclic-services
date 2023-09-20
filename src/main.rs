mod services;

fn main() {
    let services = services::Services::new();

    services.service1.do_something(3);
    services.service2.call_service_1();
}
