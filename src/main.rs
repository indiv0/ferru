extern crate serialize;
extern crate nickel;
extern crate http;
extern crate rustdoc;

use nickel::{Nickel, HttpRouter};
use std::io::net::ip::Ipv4Addr;

mod controllers;

fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/", controllers::root_handler);
    router.get("/blog/:post_year/:post_id", controllers::get_blog_post);

    server.utilize(router);

    server.handle_error(controllers::custom_errors);

    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
