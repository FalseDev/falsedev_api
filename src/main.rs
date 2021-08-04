use falsedev_api::start_server;

fn main() {
    #[cfg(not(features = "cli"))]
    start_server();
    #[cfg(features = "cli")]
    cli();
}
