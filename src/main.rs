extern crate serialize;
extern crate nickel;
extern crate http;

use nickel::{Nickel, HttpRouter};
use std::io::net::ip::Ipv4Addr;

mod controllers;

fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/", controllers::root_handler);

    server.utilize(router);

    server.handle_error(controllers::custom_errors);

    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
