#![feature(try_from, try_trait, iterator_try_fold)]

use std::convert::TryFrom;
use std::net::TcpListener;
use std::path::Path;
use std::io::Write;

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
    let mut stream = match stream {
      Ok(s) => s,
      Err(e) => {
        eprintln!("Error accepting stream: {}", e);
        continue;
      }
    };

    let parsed_string = utils::read_string_from_stream(&mut stream);

    let request_str = match parsed_string {
      Ok(s) => s,
      Err(e) => {
        eprintln!("Error parsing stream: {}", e);
        continue;
      }
    };

    match http::Request::try_from(request_str)
      .map(|req| router.handle_request(req))
      .map(|res| write!(stream, "{}", res)) {
        Ok(_) => println!("Sent response to user"),
        Err(e) => eprintln!("Error: {}", e)
    }

    /* let parsed_string = utils::read_string_from_stream(&mut stream);

    match parsed_string {
      Ok(request_str) => {
        match http::Request::try_from(request_str) {
          Ok(request) => {
            if let Ok(response) = router.handle_request(request) {
              write!(stream, "{}", response);
            }
          },
          Err(e) => eprintln!("Error parsing string to Request: {}", e),
        };
      }
      Err(e) => eprintln!("Error parsing stream: {}", e),
    } */
  }
}
