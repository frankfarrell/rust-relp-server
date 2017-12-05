#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use std::str::FromStr;
use std::io;
use std::str;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};
use regex::Regex;


//Following example here: https://tokio.rs/docs/getting-started/simple-server/


const RELP_VERSION: &'static str = "relp_version";
const COMMANDS: &'static str = "command";

enum SyslogCommand {
    Open,
    Syslog,
    Close
}

impl FromStr for SyslogCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<SyslogCommand, ()> {
        match s {
            "open" => Ok(SyslogCommand::Open),
            "syslog" => Ok(SyslogCommand::Syslog),
            "close" => Ok(SyslogCommand::Close),
            _ => Err(())
        }
    }

}

enum ResponseStatusCode {
    OK, ERROR
}

// https://doc.rust-lang.org/std/str/trait.FromStr.html
impl FromStr for ResponseStatusCode {

    type Err = ();

    fn from_str(s: &str) -> Result<ResponseStatusCode, ()> {
        match s {
            "200" => Ok(ResponseStatusCode::OK),
            "500" => Ok(ResponseStatusCode::ERROR),
            _ => Err(())
        }
    }
}

//https://stackoverflow.com/questions/33925232/how-to-match-over-self-in-an-enum
impl ToString for ResponseStatusCode {
    fn to_string(&self) -> String {
        match *self {
            ResponseStatusCode::OK => "200".to_string(),
            ResponseStatusCode::ERROR => "500".to_string()
        }
    }
}


pub struct RelpSyslogMessage {
    //Needs to be between 1 and 999_999_999
    transaction_number: usize,
    command : SyslogCommand,
    /*
    Number of bytes in data
    Needs to be between 1 and 999_999_999
    Typically only up to 128KB?
     */
    data_length: usize,
    //Can this be empty? Eg if data length is zero?
    data: Option<String>
}

pub struct RelpResponse {
    //Needs to be between 1 and 999_999_999
    transaction_number: usize,
    status : ResponseStatusCode,
    message: Option<String>
}

pub struct RelpCodec;

impl Decoder for RelpCodec {
    type Item = RelpSyslogMessage;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<RelpSyslogMessage>> {

        lazy_static! {
            static ref RELP_PATTERN: Regex = Regex::new(r"^(?P<transactionNumber>\\d{1,9}) (?P<command>[a-zA-Z]{1,32}) (?P<dataLen>\\d{1,9})( (?P<data>.+))?").unwrap();
        }

        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.split_to(i);

            // Also remove the '\n'
            buf.split_to(1);

            // Turn this data into a UTF string and return it in a Frame.
            match str::from_utf8(&line) {
                Ok(s) => match RELP_PATTERN.is_match(s) {
                    true => {
                        let caps = RELP_PATTERN.captures(s).unwrap();
                        Ok(Some(RelpSyslogMessage {
                            transaction_number: caps.name("transactionNumber").unwrap().as_str().parse::<usize>().unwrap(),
                            command: SyslogCommand::from_str(caps.name("command").unwrap().as_str()).unwrap(),
                            data_length: caps.name("dataLen").unwrap().as_str().parse::<usize>().unwrap(),
                            data: caps.name("data").map(|d| d.as_str().to_string()),
                        }))
                    },
                    false => Err(io::Error::new(io::ErrorKind::Other,
                                                "Not a valid RELP Frame"))
                }
                Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                             "invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
    }
}

impl Encoder for RelpCodec {
    type Item = RelpResponse;
    type Error = io::Error;

    fn encode(&mut self, relp_response: RelpResponse, buf: &mut BytesMut) -> io::Result<()> {

        //Is this a good way to build a string?
        let response_string: String = [
                relp_response.transaction_number.to_string(),
                " ".to_string(),
                "rsp".to_string(),
                " ".to_string(),
                relp_response.status.to_string(),
                relp_response.message.map(|m| " ".to_string()+ &m )
                    .unwrap_or("".to_string())
            ]
            .concat();

        buf.extend(response_string.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}


//TODO Service:

fn main() {
}
