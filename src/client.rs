extern crate tokio;
extern crate ws;

mod event;

use deku::prelude::*;
use std::error::Error;
use std::sync::mpsc;
use tokio::io::{self, AsyncBufReadExt};
use ws::{connect, Handler, Handshake, Message, Result, Sender};

use event::WsEvents;

struct Client {
    out: Sender,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("opened");
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Client got message '{}'. ", msg);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    let (tx, rx) = mpsc::channel();

    tokio::spawn(async move {
        if let Err(error) = connect("ws://127.0.0.1:3012", |out| {
            tx.send(out.clone()).expect("failed to send 'out'");

            Client { out }
        }) {
            println!("Failed to create WebSocket due to: {:?}", error);
        }
    });

    let ws_sender = rx.recv().unwrap();

    loop {
        tokio::select! {
            line = stdin.next_line() => {
                let line = line?.expect("stdin closed");
                let msg = WsEvents::SendMessage {
                    len: line.len() as u8,
                    text: line.as_bytes().to_vec()
                };

                ws_sender.send(msg.to_bytes().unwrap()).unwrap();
            }
        }
    }
}
