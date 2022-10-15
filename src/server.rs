extern crate ws;

mod event;

use clap::Parser;
use event::WsEvents;
use std::fs;
use std::path::Path;
use ws::{listen, Handler, Message, Sender};

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(long)]
    host: String,

    #[arg(long, short)]
    shares: String,
}

struct Server {
    out: Sender,
    shares_path: String,
}

impl Server {
    fn handle_incoming_message(&mut self, bin: Vec<u8>) {
        let event = WsEvents::try_from(&bin[..]).unwrap();

        match event {
            WsEvents::CreateShare { name, .. } => {
                let name = String::from_utf8(name).unwrap();
                let path = Path::new(&self.shares_path).join(name);

                if let Err(e) = fs::File::create(&path) {
                    println!("Failed to create file due to: {}", e);
                } else {
                    self.out
                        .send(get_shares(self.shares_path.clone()).as_bytes())
                        .unwrap();
                }
            }
            WsEvents::GetShares => {
                self.out
                    .send(get_shares(self.shares_path.clone()).as_bytes())
                    .unwrap();
            }
        }
    }
}

fn get_shares(shares_path: String) -> String {
    let mut out = String::new();

    let dir = fs::read_dir(shares_path).unwrap();
    for path in dir {
        let path = path.unwrap().path().display().to_string();

        out.push_str(&path);
        out.push('\n');
    }

    out
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
        shares_path: args.shares.clone(),
    }) {
        println!("Failed to create WebSocket due to {:?}", error);
    }
}
