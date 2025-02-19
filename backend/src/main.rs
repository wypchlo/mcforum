use api::server;

fn main() {
    env_logger::init();
    server::start().unwrap();
}
