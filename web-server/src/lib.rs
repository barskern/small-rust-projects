use std::net::TcpListener;
use std::io::Read;
use std::path::Path;

mod utils;
mod http;
mod router;

use router::Router;

const MAX_REQUEST_SIZE: usize = 512;

pub fn run(port: usize) {
  let dir_path = Path::new("./html/");
  let router = Router::new(dir_path);

  let listener = TcpListener::bind(format!("localhost:{}", port)).unwrap();
  println!("Listening for connections at port {}", port);
  for stream in listener.incoming() {
    match stream {
      Ok(mut stream) => {
        let mut buffer: [u8; MAX_REQUEST_SIZE] = [0; MAX_REQUEST_SIZE];
        let n_bytes_read = stream.read(&mut buffer).unwrap();
        let request_str: &str = std::str::from_utf8(&buffer[..n_bytes_read]).unwrap();

        let request = http::Request::from_str(request_str).unwrap();
        println!("{:#?}", request);
        router.handle_request(request);
      }
      Err(e) => {
        println!("Connection failed: {}", e);
      }
    }
  }
}
