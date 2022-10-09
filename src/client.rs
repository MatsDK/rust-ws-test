extern crate env_logger;
extern crate ws;

use ws::{connect, CloseCode};

fn main() {
    env_logger::init();

    if let Err(error) = connect("ws://127.0.0.1:3012", |out| {
        if out.send("Hello WebSocket").is_err() {
            println!("Websocket couldn't queue an initial message.")
        } else {
            println!("Client sent message 'Hello WebSocket'. ")
        }

        move |msg| {
            println!("Client got message '{}'. ", msg);

            out.close(CloseCode::Normal)
        }
    }) {
        println!("Failed to create WebSocket due to: {:?}", error);
    }
}