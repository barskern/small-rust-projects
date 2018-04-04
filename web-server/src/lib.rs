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
    /*     stream
      .map_err(|e| eprintln!("Connection failed: {}", e))
      .map(|mut stream| utils::read_string_from_stream(&mut stream))
      .map_err(|e| eprintln!("Parsing string from TcpStream failed: {}", e))
      .map(|request_str| http::Request::try_from(request_str)); */
    match stream {
      Ok(mut stream) => {
        match utils::read_string_from_stream(&mut stream) {
          Ok(request_str) => match http::Request::try_from(request_str) {
            Ok(request) => {
              println!("{:#?}", request);
              router.handle_request(request);
            }
            Err(e) => eprintln!("Parsing string to Request failed: {}", e),
          },
          Err(e) => {
            /* Perhaps do some logging of failed requests? */
            eprintln!("Parsing string from TcpStream failed: {}", e);
          }
        }
      }
      Err(e) => {
        /* Perhaps do some logging of failed requests? */
        eprintln!("Connection failed: {}", e);
      }
    }
  }
}
