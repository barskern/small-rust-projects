#![feature(try_from)]

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
    match stream {
      Ok(mut stream) => {
        match utils::read_string_from_stream(&mut stream) {
          Ok(request_str) => {
            println!("Recived request:\n{}", request_str);
            /* let request: http::Request = request_str.parse().unwrap();
            println!("{:#?}", request);
            router.handle_request(request); */
          }
          Err(e) => {
            /* Perhaps do some logging of failed requests? */
            println!("Parsing of request failed: {}", e);
          }
        }
      }
      Err(e) => {
        /* Perhaps do some logging of failed requests? */
        println!("Connection failed: {}", e);
      }
    }
  }
}
