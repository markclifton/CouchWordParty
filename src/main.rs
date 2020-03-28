extern crate ws;
use std::io::Read;
use ws::{listen, Handler, Message, Request, Response, Result, Sender};

struct Server {
    out: Sender,
}

impl Handler for Server {
    fn on_request(&mut self, req: &Request) -> Result<Response> {
        // Using multiple handlers is better (see router example)
        match req.resource() {
            // The default trait implementation
            "/ws" => Response::from_request(req),

            // Create a custom response
            "/" => Ok(Response::new(
                200,
                "OK",
                get_file_as_byte_vec(&String::from("resources/html/activeSCreen.html")),
            )),

            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    // Handle messages recieved in the websocket (in this case, only on /ws)
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Broadcast to all connections
        self.out.broadcast(msg)
    }
}

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = std::fs::File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn main() {
    // Listen on an address and call the closure for each connection
    listen("192.168.1.2:8000", |out| Server { out }).unwrap()
}
