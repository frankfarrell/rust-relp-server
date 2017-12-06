extern crate regex;
extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use model::{ResponseStatusCode,RelpResponse, RelpSyslogMessage, SyslogCommand};

//use relp::codec;

use std::str::FromStr;
use std::io;
use std::str;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};
use regex::Regex;
use std::io::Write;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RelpCodec(());

impl RelpCodec {
    /// Creates a new `BytesCodec` for shipping around raw bytes.
    pub fn new() -> RelpCodec { RelpCodec(())  }
}

impl Decoder for RelpCodec {
    type Item = RelpSyslogMessage;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<RelpSyslogMessage>> {

        lazy_static! {
            static ref RELP_PATTERN: Regex = Regex::new(r"^(?P<transactionNumber>\d{1,9}) (?P<command>[a-zA-Z]{1,32}) (?P<dataLen>\d{1,9})( (?P<data>.+))?").unwrap();
        }

        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.split_to(i);

            // Also remove the '\n'
            buf.split_to(1);

            // Turn this data into a UTF string and return it in a Frame.
            match str::from_utf8(&line) {
                Ok(s) => {
                    println!("RELP Message received:{}", s);
                    io::stdout().flush().unwrap();
                    match RELP_PATTERN.is_match(s) {
                        true => {
                            println!("It matches");
                            io::stdout().flush().unwrap();
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


#[cfg(test)]
mod tests {

    use std::str;
    use bytes::BytesMut;
    use bytes::BufMut;
    use super::*;
    use tokio_io::codec::Decoder;
    use tokio_io::codec::Encoder;
    use model::{ResponseStatusCode,RelpResponse, RelpSyslogMessage, SyslogCommand};

    #[test]
    fn it_decodes_message_correctly() {
        let mut relp_codec = RelpCodec::new();
        let buf = &mut BytesMut::new();

        buf.put_slice(b"12 open 15 LOTS,OF,data123\n");

        assert_eq!(RelpSyslogMessage{
            transaction_number: 12,
            command: SyslogCommand::Open,
            data_length: 15,
            data: Some("LOTS,OF,data123".to_string()),
        }, relp_codec.decode(buf).unwrap().unwrap());
    }

    #[test]
    fn it_encodes_message_correctly() {
        let mut relp_codec = RelpCodec::new();
        let buf = &mut BytesMut::new();

        let response = RelpResponse{
            transaction_number: 4,
            status: ResponseStatusCode::OK,
            message: None,
        };

        relp_codec.encode(response, buf);

        assert_eq!("4 rsp 200\n", str::from_utf8(&buf).unwrap());
    }
}