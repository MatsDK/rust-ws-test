use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum WsEvents {
    #[deku(id = "0")]
    CreateShare {
        len: u8,
        #[deku(count = "len", endian = "big")]
        name: Vec<u8>,
    },
    #[deku(id = "1")]
    GetShares,
}
