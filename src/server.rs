extern crate ws;

mod event;

use clap::Parser;
use event::WsEvents;
use std::fs;
use ws::{listen, Handler, Message, Sender};

#[derive(Parser)]
#[command()]
struct Args {
    #[arg(long)]
    host: String,

    #[arg(long, short)]
    shares: String,
}

struct Server {
    out: Sender,
    shares: String,
}

impl Server {
    fn handle_incoming_message(&mut self, bin: Vec<u8>) {
        let event = WsEvents::try_from(&bin[..]).unwrap();

        match event {
            WsEvents::SendMessage { text, .. } => {
                println!("received message: {}", String::from_utf8(text).unwrap());
                self.out.send("got message").unwrap();
            }
            WsEvents::GetShares => {
                let dir = fs::read_dir(self.shares.clone()).unwrap();

                let mut out = String::new();

                for path in dir {
                    let path = path.unwrap().path();
                    if !path.is_dir() {
                        continue;
                    }

                    let path_str = path.to_str().unwrap();

                    out.push_str(path_str);
                    out.push('\n');
                }

                self.out.send(out.as_bytes()).unwrap();
            }
        }
    }
}

impl Handler for Server {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        println!("connection opened");
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        println!("received msg: {:?}", msg);

        if let Message::Binary(bin) = msg {
            self.handle_incoming_message(bin)
        }

        Ok(())
    }
}

fn main() {
    let args = Args::parse();

    if let Err(error) = listen(args.host, |out| Server {
        out,
        shares: args.shares.clone(),
    }) {
        println!("Failed to create WebSocket due to {:?}", error);
    }
}
