mod services;

fn main() {
    let services = services::Services::new();

    services.service2().call_service_1();
}
