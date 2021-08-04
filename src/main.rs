use falsedev_api::{start_server, ServerState};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref STATE: ServerState = {
        println!("Lol");
        ServerState::new("./config/config.toml")
    };
}

fn main() {
    println!("Pre lol?");
    #[cfg(not(features = "cli"))]
    {
        start_server(&STATE);
    }
    #[cfg(features = "cli")]
    cli();
}
