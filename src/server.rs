use crate::{fairings::response_time::RequestTimer, state::serverstate::ServerState};

lazy_static::lazy_static! {
    static ref STATE: ServerState = {
        ServerState::new("./config/config.toml")
    };
}

fn create_server() -> rocket::Rocket<rocket::Build> {
    let state: &ServerState = &STATE;
    rocket::build()
        .mount(
            "/",
            crate::routes::image::manipulation::image_manipulation_routes(),
        )
        .mount(
            "/template",
            routes![crate::routes::image::templates::template],
        )
        .manage(state)
        .attach(RequestTimer)
}

pub fn start_server() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { create_server().launch().await })
        .unwrap();
}
