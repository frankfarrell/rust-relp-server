use tokio_proto::pipeline::ServerProto;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use model::{ResponseStatusCode,RelpResponse, RelpSyslogMessage, SyslogCommand};
use codec::RelpCodec;
use std::io::{self, ErrorKind, Write, Error};

pub struct RelpProtocol;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for RelpProtocol {
    // For this protocol style, `Request` matches the `Item` type of the codec's `Decoder`
    type Request = RelpSyslogMessage;

    // For this protocol style, `Response` matches the `Item` type of the codec's `Encoder`
    type Response = RelpResponse;

    // A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, RelpCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(RelpCodec::new()))
    }
}