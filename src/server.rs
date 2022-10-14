extern crate ws;

mod event;

use event::WsEvents;
use ws::{listen, Message};

fn main() {
    if let Err(error) = listen("127.0.0.1:3012", |out| {
        move |msg| {
            if let Message::Binary(bin) = msg {
                let event = WsEvents::try_from(&bin[..]).unwrap();

                match event {
                    WsEvents::SendMessage { text, .. } => {
                        println!("received message: {}", String::from_utf8(text).unwrap());
                        return out.send("got message");
                    }
                }
            }

            out.send("something went wrong")
        }
    }) {
        println!("Failed to create WebSocket due to {:?}", error);
    }
}
