use http::server::request::RequestUri::AbsolutePath;
use http::method::Method::{Get, Head};
use http::status::Status::InternalServerError;
use nickel::{
    Middleware,
    MiddlewareResult,
    Request,
    Response,
};
use nickel::mimes::MediaType;
use nickel::{ErrorWithStatusCode, NickelError};
use nickel::Action::{Halt, Continue};
use std::io::{IoError, IoResult, FileNotFound};
use std::collections::HashMap;

use post;

pub struct MarkdownPageHandler {
    root_path: Path,
    template_path: &'static str
}

impl Middleware for MarkdownPageHandler {
    fn invoke (&self, req: &mut Request, res: &mut Response)
                -> MiddlewareResult {
        match req.origin.method {
            Get | Head => {
                match self.with_file(self.extract_path(req), res) {
                    Ok(()) => Ok(Halt),
                    Err(err) => match err.kind {
                        // We shouldn't assume the MarkdownPageHandler to be the last middleware in the stack.
                        // Therefore it's important to continue in case of FileNotFound errors.
                        FileNotFound => Ok(Continue),
                        _ => Err(NickelError::new(format!("Unknown Error ({})", err),
                        ErrorWithStatusCode(InternalServerError)))
                    }
                }
            },
            _ => Ok(Continue)
        }
    }
}

impl MarkdownPageHandler {
    /// Create a new middleware to serve markdown files from within a given root
    /// directory using the provided template.
    /// The file to serve will be determined by combining the requested Url with
    /// the provided root directory.
    ///
    ///
    /// # Example
    /// ```{rust}
    /// use nickel::Nickel;
    /// use ferrum::MarkdownPageHandler;
    /// let mut server = Nickel::new();
    ///
    /// server.utilize(MarkdownPageHandler::new("/path/to/serve/"));
    /// ```
    pub fn new (root_path: &str, template_path: &'static str) -> MarkdownPageHandler {
        MarkdownPageHandler {
            root_path: Path::new(root_path),
            template_path: template_path
        }
    }

    fn extract_path<'a>(&self, req: &'a mut Request) -> Option<&'a str> {
        match req.origin.request_uri {
            AbsolutePath(ref path) => {
                debug!("{} {}{}", req.origin.method, self.root_path.display(), path);
                match path.as_slice() {
                    "/" => Some("index"),
                    path => Some(path.slice_from(1)),
                }
            }
            _ => None
        }
    }

    fn with_file(&self, relative_path: Option<&str>, res: &mut Response)
        -> IoResult<()> {
        match relative_path {
            Some(path) => {
                /*let mut template_type: Option<&str> = None;
                let path = if path.contains("/") {
                    let mut split_path = path.splitn(1, '/');
                    let template_type = split_path.next().unwrap();
                    split_path.next().unwrap()
                } else { path };
                let page_path = format!("{}/{}.md", self.root_path, path);*/
                let page_path = self.root_path.join(format!("{}.md", path));

                let page = match post::load_from_disk(&page_path) {
                    Ok(page) => page,
                    Err(e) => {
                        /*error!("Failed to parse page: {}", e);
                        util::handle_error(InternalServerError, "500", "An internal error has occurred!", res);
                        return Err(e)*/
                        error!("Failed to parse page: {}", e);
                        return Err(IoError::last_error())
                    }
                };
                let page_content = page.render();

                let mut data = HashMap::<&str, &str>::new();
                data.insert("content", page_content.as_slice());
                res.content_type(MediaType::Html)
                   .render(self.template_path, &data);
                Ok(())
            },
            None => Err(IoError::last_error())
        }
    }
}
