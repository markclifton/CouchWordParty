#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate json;

use rocket_contrib::serve::StaticFiles;

use ws::{listen, Handler, Message, Request, Response, Result, Sender};

use std::sync::atomic::{AtomicI32, Ordering};
static NEXT_PLAYER_ID: AtomicI32 = AtomicI32::new(0);

static LOGIN_HTML: &'static [u8] =
    br#"<div style='overflow:auto;background-color:#f1f1f1;padding:15px;text-align: center;'>
        <label for='name'>Name:</label>
        <input type='text' id='name' name='name'><br><br>
        <label for='room'>Room:</label>
        <input type='text' id='room' name='room'><br><br>
        <button style='margin-left: 2.5px;'>Create</button>
        <button onclick='submit()'>Join</button>
    </div>
"#;

static WAIT_HTML: &'static [u8] =
    br#"<div style='overflow:auto;background-color:#f1f1f1;padding:15px;text-align: center;'>
        <h1>Waiting for players to join...</h1>
    </div>
"#;

struct WS {
    out: Sender,
    id: i32,
}
use std::io::Read;
impl Handler for WS {
    fn on_request(&mut self, req: &Request) -> Result<Response> {
        match req.resource() {
            "/ws" => {
                self.id = NEXT_PLAYER_ID.fetch_add(1, Ordering::SeqCst);
                self.out.send(std::str::from_utf8(LOGIN_HTML).unwrap());
                Response::from_request(req)
            }

            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        //if (msg.as_text().unwrap() == "wait") {
        self.out.send(std::str::from_utf8(WAIT_HTML).unwrap())

        //self.out.broadcast(msg)
    }
}

#[get("/")]
fn index() -> Option<rocket::response::NamedFile> {
    rocket::response::NamedFile::open("resources/html/index.html").ok()
}

#[get("/<file..>")]
fn files(file: std::path::PathBuf) -> Option<rocket::response::NamedFile> {
    rocket::response::NamedFile::open(file).ok()
}

fn main() {
    std::thread::spawn(move || listen("192.168.1.2:7777", |out| WS { out: out, id: -1 }).unwrap());

    rocket::ignite()
        .mount("/", StaticFiles::from("/resources"))
        .mount("/", routes![index, files])
        .launch();
}
