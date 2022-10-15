extern crate tokio;
extern crate ws;

mod event;

use clap::Parser;
use deku::prelude::*;
use std::error::Error;
use std::sync::mpsc;
use tokio::io::{self, AsyncBufReadExt};
use ws::{connect, Handler, Handshake, Message, Result, Sender};

use event::WsEvents;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(long)]
    host: String,
}

struct Client {
    out: Sender,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("opened");
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("SHARES:\n{}", msg);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut stdin = io::BufReader::new(io::stdin()).lines();

    let (tx, rx) = mpsc::channel();

    tokio::spawn(async move {
        if let Err(error) = connect(args.host, |out| {
            tx.send(out.clone()).expect("failed to send 'out'");

            Client { out }
        }) {
            println!("Failed to create WebSocket due to: {:?}", error);
        }
    });

    let ws_sender = rx.recv().unwrap();

    let msg = WsEvents::GetShares;
    ws_sender.send(msg.to_bytes().unwrap()).unwrap();

    loop {
        tokio::select! {
            line = stdin.next_line() => {
                let line = line?.expect("stdin closed");
                let msg = WsEvents::CreateShare {
                    len: line.len() as u8,
                    name: line.as_bytes().to_vec()
                };

                ws_sender.send(msg.to_bytes().unwrap()).unwrap();

                let get_shares_msg = WsEvents::GetShares;
                ws_sender.send(get_shares_msg.to_bytes().unwrap()).unwrap();
            }
        }
    }
}
