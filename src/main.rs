use cgi::{http::StatusCode, Request, Response};

fn main() {
    cgi::handle(handler);
}

fn handler(request: Request) -> Response {
    let who = String::from_utf8_lossy(request.body());
    let who = if who.trim().is_empty() {
        "World"
    } else {
        who.trim()
    };

    cgi::text_response(StatusCode::OK, format!("Hello, {who}!"))
}
