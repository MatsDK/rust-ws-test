extern crate tokio;
extern crate ws;

use std::error::Error;
use tokio::io::{self, AsyncBufReadExt};
use ws::{connect, Handler, Handshake, Message, Result, Sender};

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

    tokio::spawn(async move {
        if let Err(error) = connect("ws://127.0.0.1:3012", |out| {
            Client { out }
        }) {
            println!("Failed to create WebSocket due to: {:?}", error);
        }
    });

    loop {
        tokio::select! {
             line = stdin.next_line() => {
                let line = line?.expect("stdin closed");
                println!("{}", line);
             }
        }
    }
}
