use std::net::TcpListener;
use std::io::Read;

mod router;
use router::Router;
use std::path::Path;

const MAX_REQUEST_SIZE: usize = 512;

pub fn run(port: usize) {
  let listener = TcpListener::bind(format!("localhost:{}", port)).unwrap();
  let dir_path = Path::new("./html/");
  let router = Router::new(dir_path);

  println!("Running webserver at port {}", port);
  for stream in listener.incoming() {
    match stream {
      Ok(mut stream) => {
        let mut buffer = [0; MAX_REQUEST_SIZE];
        stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer);
        router.handle_request(&request);
      }
      Err(e) => {
        println!("Connection failed: {}", e);
      }
    }
  }
}
