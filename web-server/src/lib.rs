#![feature(try_from)]

use std::convert::TryFrom;
use std::net::TcpListener;
use std::path::Path;

mod http;
mod router;
mod utils;

use router::Router;

pub fn run(port: usize) {
  let dir_path = Path::new("./html/");
  let router = Router::new(dir_path);

  let listener = TcpListener::bind(format!("localhost:{}", port)).unwrap();
  println!("Listening for connections at port {}", port);

  for stream in listener.incoming() {
    let parsed_stream = stream
      .map_err(utils::ReadStreamError::Io)
      .and_then(|mut stream| utils::read_string_from_stream(&mut stream));

    match parsed_stream {
      Ok(request_str) => {
        match http::Request::try_from(request_str) {
          Ok(request) => router.handle_request(request),
          Err(e) => eprintln!("Error parsing string to Request: {}", e),
        };
      }
      Err(e) => eprintln!("Error parsing stream: {}", e),
    }
  }
}
