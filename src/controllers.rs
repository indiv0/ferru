use http::status::Status;
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
use rustdoc::html::markdown::Markdown;
use std::collections::HashMap;
use std::io::{
    BufferedReader,
    File
};

/// Render the index page.
pub fn root_handler(_request: &Request, response: &mut Response) {
    let mut data = HashMap::<&str, &str>::new();
    data.insert("name", "user");
    response.render("assets/templates/index.tpl", &data);
}

pub fn get_blog_post(request: &Request, response: &mut Response) {
    // Retrieve the post id from the request URL.
    let post_year = request.param("post_year");
    let post_id = request.param("post_id");
    let post_path = Path::new("assets/content/posts/".to_string() +
            post_year.to_string() +
            "/".to_string() +
            post_id.to_string() +
            ".md".to_string());

    // Read the post markdown from the disk.
    let content = File::open(&post_path).read_to_end().unwrap();
    let content = String::from_utf8(content).unwrap();
    let mut content = content.as_slice().split_str("\n\n");
    //let mut file = BufferedReader::new(File::open(&post_path));
    //let mut contents = "";
    let heading = content.next().unwrap();
    let mut content = content.map(|x| x.to_string());
    let content = content.fold("".to_string(), |a, b| a + "\n\n".to_string() + b);
    /*for line in file.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.as_slice().rsplitn(1, ": ").collect();
        match split.get(1) {
            Ok(value) => {
                println!("value");
            }
            Err(e) => {
                contents.push_str(split.get(0));
            }
        }
        //print!("{}", line.unwrap());
        println!("{}", split);
    }*/
    let html_content = format!("{}", Markdown(content.as_slice()));

    let mut data = HashMap::<&str, &str>::new();
    data.insert("content", html_content.as_slice());
    response.render("assets/templates/post.tpl", &data);
}

pub fn custom_errors(err: &NickelError, _req: &Request, response: &mut Response) -> MiddlewareResult {
    use http::status::Status::{Forbidden, NotFound, InternalServerError};

    match err.kind {
        ErrorWithStatusCode(Forbidden) => {
            handle_error(Forbidden, "403", "You are not authorized to view this page", response);
            Ok(Halt)
        },
        ErrorWithStatusCode(NotFound) => {
            handle_error(NotFound, "404", "That file could not be found.", response);
            Ok(Halt)
        },
        ErrorWithStatusCode(InternalServerError) => {
            handle_error(NotFound, "500", "An error has occured!", response);
            Ok(Halt)
        },
        _ => Ok(Continue)
    }
}

fn handle_error(status_code: Status, error_code: &str, error_message: &str, res: &mut Response) {
    let mut data = HashMap::<&str, &str>::new();

    data.insert("site_url", "http://nikitapek.in");
    data.insert("error_code", error_code);
    data.insert("error_message", error_message);

    res.content_type(MediaType::Html)
       .status_code(status_code)
       .render("assets/templates/error.tpl", &data);
}
