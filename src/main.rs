#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;


mod model;
use model::{ResponseStatusCode,RelpResponse, RelpSyslogMessage, SyslogCommand};

mod codec;

use std::str::FromStr;
use std::io;
use std::str;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};
use regex::Regex;


//Following example here: https://tokio.rs/docs/getting-started/simple-server/

const RELP_VERSION: &'static str = "relp_version";
const COMMANDS: &'static str = "command";

//TODO Service:

fn main() {
}

