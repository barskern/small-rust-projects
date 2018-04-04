#![feature(try_from)]

use std::net::TcpListener;
use std::io::Read;
use std::path::Path;

mod utils;
mod http;
mod router;

use router::Router;

const MAX_REQUEST_SIZE: usize = 1024;

pub fn run(port: usize) {
  let dir_path = Path::new("./html/");
  let router = Router::new(dir_path);

  let listener = TcpListener::bind(format!("localhost:{}", port)).unwrap();
  println!("Listening for connections at port {}", port);
  for stream in listener.incoming() {
    match stream {
      Ok(mut stream) => {
        let mut request_str = String::with_capacity(MAX_REQUEST_SIZE);
        println!("Initialized request str");

        let _n_bytes_read = stream
          .take(MAX_REQUEST_SIZE as u64)
          .read_to_string(&mut request_str)
          .unwrap();

        println!("{}", request_str);

        /* let request: http::Request = request_str.parse().unwrap();
        println!("{:#?}", request);
        router.handle_request(request); */
      }
      Err(e) => {
        println!("Connection failed: {}", e);
      }
    }
  }
}
