use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum WsEvents {
    #[deku(id = "0")]
    SendMessage {
        len: u8,
        #[deku(count = "len", endian = "big")]
        text: Vec<u8>,
    },
}
