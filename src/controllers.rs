use http::status::{Forbidden, NotFound, InternalServerError};
use nickel::{
    Continue,
    ErrorWithStatusCode,
    Halt,
    MiddlewareResult,
    NickelError,
    Request,
    Response
};
use std::collections::HashMap;

pub fn root_handler(_request: &Request, response: &mut Response) {
    let mut data = HashMap::<&str, &str>::new();
    data.insert("name", "user");
    response.render("assets/templates/index.tpl", &data);
}

pub fn custom_errors(err: &NickelError, _req: &Request, response: &mut Response) -> MiddlewareResult {
    match err.kind {
        ErrorWithStatusCode(Forbidden) => {
            let mut data = HashMap::<&str, &str>::new();
            data.insert("error_code", "403");
            data.insert("error_message", "You are not authorized to view this page.");
            data.insert("site_url", "http://nikitapek.in");

            response.content_type("html")
                    .status_code(Forbidden)
                    .render("assets/templates/error.tpl", &data);
            Ok(Halt)
        },
        ErrorWithStatusCode(NotFound) => {
            let mut data = HashMap::<&str, &str>::new();
            data.insert("error_code", "404");
            data.insert("error_message", "That file could not be found.");
            data.insert("site_url", "http://nikitapek.in");

            response.content_type("html")
                    .status_code(NotFound)
                    .render("assets/templates/error.tpl", &data);
            Ok(Halt)
        },
        ErrorWithStatusCode(InternalServerError) => {
            let mut data = HashMap::<&str, &str>::new();
            data.insert("error_code", "500");
            data.insert("error_message", "An error has occured!");
            data.insert("site_url", "http://nikitapek.in");

            response.content_type("html")
                    .status_code(InternalServerError)
                    .render("assets/templates/error.tpl", &data);
            Ok(Halt)
        },
        _ => Ok(Continue)
    }
}
