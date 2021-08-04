use crate::{fairings::response_time::RequestTimer, state::serverstate::ServerState};

fn create_server(server_state: &'static ServerState) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount(
            "/",
            crate::routes::image::manipulation::image_manipulation_routes(),
        )
        .mount(
            "/template",
            routes![crate::routes::image::templates::template],
        )
        .manage(server_state)
        .attach(RequestTimer)
}

pub fn start_server(server_state: &'static ServerState) {
    rocket::tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { create_server(server_state).launch().await })
        .unwrap();
}
