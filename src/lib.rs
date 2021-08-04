#[macro_use]
extern crate rocket;

mod datastructures;
mod errors;
mod fairings;
mod imagelib;
mod routes;
mod server;
mod state;

pub use server::start_server;
