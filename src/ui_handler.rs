use std::borrow::Cow;
use wry::http::{Request, Response, header};
use crate::assets::Assets;

pub fn handle_zenith_request(_ui_html: &str, request: Request<Vec<u8>>) -> Response<Cow<'static, [u8]>> {
    let uri = request.uri();
    let host = uri.host().unwrap_or_default();
    let original_path = uri.path();

    if host != "assets" {
        return Response::builder().status(404).body(Cow::Borrowed(&[][..])).unwrap();
    }

    // Modern SPA Routing: ONLY serve it for the chrome UI path
    let is_ui_request = original_path.starts_with("/ui");
    
    let path = if is_ui_request {
        "index.html".to_string()
    } else {
        original_path.trim_start_matches('/').to_string()
    };

    // Try finding the exact file or adding .html
    let mut file_path = path.clone();
    if !file_path.contains('.') && !file_path.is_empty() {
        if Assets::get(&format!("{}.html", file_path)).is_some() {
            file_path = format!("{}.html", file_path);
        } else if is_ui_request {
            file_path = "index.html".to_string();
        }
    }

    if let Some(file) = Assets::get(&file_path) {
        let mime = mime_guess::from_path(&file_path).first_or_octet_stream();
        return Response::builder()
            .header(header::CONTENT_TYPE, mime.to_string())
            .body(Cow::Owned(file.data.into_owned()))
            .unwrap();
    }

    // Ultimate fallback for Chrome UI only
    if is_ui_request {
        if let Some(index) = Assets::get("index.html") {
            return Response::builder()
                .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                .body(Cow::Owned(index.data.into_owned()))
                .unwrap();
        }
    }

    Response::builder()
        .status(404)
        .body(Cow::Borrowed(&[][..])).unwrap()
}
