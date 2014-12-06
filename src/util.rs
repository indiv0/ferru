use http::status::Status;
use nickel::Response;
use nickel::mimes::MediaType;
use std::collections::HashMap;

pub fn handle_error(status_code: Status, error_code: &str, error_message: &str, res: &mut Response) {
    let mut data = HashMap::<&str, &str>::new();

    data.insert("site_url", "http://nikitapek.in");
    data.insert("error_code", error_code);
    data.insert("error_message", error_message);

    res.content_type(MediaType::Html)
       .status_code(status_code)
       .render("assets/templates/error.tpl", &data);
}
