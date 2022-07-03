use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
use string_join::Join;

fn format_output(board: &str, pretty: bool) -> &str {
    if pretty {
        board.chars().join()
        format!("{} | {} | {}", board[i], board[i+1], board[i+2]).into()
    } else {
        board
    }
}

/// A simple Spin HTTP component.
#[http_component]
fn spin_tac_toe(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    let test_header = req.headers().get("test").is_some();
    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .header("test", if test_header { "YEP" } else { "NOPE" })
        .body(Some("Hello, Fermyon".into()))?)
}
