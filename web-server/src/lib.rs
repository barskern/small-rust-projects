#![feature(try_from, try_trait, iterator_try_fold)]

mod http;
mod router;
mod utils;
mod errors;

use std::{convert::TryFrom, io::Write, net::{TcpListener, TcpStream}, path::Path};
use router::Router;
use errors::HandleStreamError;
use http::ParseHttpError;

pub fn run(port: usize) {
  let dir_path = Path::new("./html/");
  let router = Router::new(dir_path);

  let listener = TcpListener::bind(format!("localhost:{}", port))
    .expect("Unable to start listening for TCP-packets.");

  println!("Listening for connections at port {}", port);

  for stream in listener.incoming() {
    match stream
      .map_err(HandleStreamError::from)
      .and_then(|s| handle_stream(&router, s)) {
      Ok(_) => println!("Sent response to user"),
      Err(e) => eprintln!("Error: {}", e)
    };
  }
}

fn handle_stream(r: &Router, mut s: TcpStream) -> Result<(), HandleStreamError> {
  let request_str = utils::read_string_from_stream(&mut s)?;

  http::Request::try_from(request_str)
    .map_err(ParseHttpError::from)
    .map_err(HandleStreamError::from)    
    .map(|req| r.handle_request(req))
    .and_then(|res| write!(s, "{}", res).map_err(HandleStreamError::from))
}
