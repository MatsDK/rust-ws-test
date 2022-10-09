extern crate ws;

use ws::listen;

fn main() {
    env_logger::init();

    if let Err(error) = listen("127.0.0.1:3012", |out| {
        move |msg| {
            println!("Server got message '{}'. ", msg);

            out.send(msg)
        }
    }) {
        println!("Failed to create WebSocket due to {:?}", error);
    }
}