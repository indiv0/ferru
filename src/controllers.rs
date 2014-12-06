use http::status::Status;
use http::status::Status::{Forbidden, NotFound, InternalServerError};
use nickel::{
    Continue,
    ErrorWithStatusCode,
    Halt,
    MiddlewareResult,
    NickelError,
    Request,
    Response,
};
use nickel::mimes::MediaType;
use std::collections::HashMap;

use post;
use util;

/// Render the index page.
pub fn root_handler(_req: &Request, res: &mut Response) {
    let mut data = HashMap::<&str, &str>::new();
    data.insert("name", "user");
    res.render("assets/templates/index.tpl", &data);
}

pub fn get_blog_post(req: &Request, res: &mut Response) {
    // Retrieve the post id from the request URL.
    let post_year = req.param("post_year");
    let post_id = req.param("post_id");

    let page_path = format!("assets/content/posts/{}/{}.md", post_year, post_id);

    handle_rendered_page(page_path.as_slice(), "assets/templates/post.tpl", res);
}

pub fn get_page(req: &Request, res: &mut Response) {
    // Retrieve the page id from the request URL.
    let page_id = req.param("page_id");

    let page_path = format!("assets/content/pages/{}.md", page_id);

    handle_rendered_page(page_path.as_slice(), "assets/templates/post.tpl", res);
}

pub fn custom_errors(err: &NickelError, _req: &Request, res: &mut Response) -> MiddlewareResult {
    match err.kind {
        ErrorWithStatusCode(Forbidden) => {
            util::handle_error(Forbidden, "403", "You are not authorized to view this page", res);
            Ok(Halt)
        },
        ErrorWithStatusCode(NotFound) => {
            util::handle_error(NotFound, "404", "That file could not be found.", res);
            Ok(Halt)
        },
        ErrorWithStatusCode(InternalServerError) => {
            util::handle_error(InternalServerError, "500", "An error has occured!", res);
            Ok(Halt)
        },
        _ => Ok(Continue)
    }
}

fn handle_rendered_page(page_path: &str, template_path: &'static str, res: &mut Response) {
    let page = match post::load_from_disk(&Path::new(page_path)) {
        Ok(page) => page,
        Err(e) => {
            error!("Failed to parse page: {}", e);
            util::handle_error(InternalServerError, "500", "An internal error has occurred!", res);
            return
        }
    };
    let page_content = page.render();

    let mut data = HashMap::<&str, &str>::new();
    data.insert("content", page_content.as_slice());
    res.content_type(MediaType::Html)
       .render(template_path, &data);
}
