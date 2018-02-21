use tokio_service::Service;
use futures::{future, Future};
use model::{ResponseStatusCode,RelpResponse, RelpSyslogMessage, SyslogCommand};
use std::io::{self, ErrorKind, Write, Error};
use std::cell::Cell;

pub struct RelpService{
    transaction_number : Cell<usize>,
    is_connection_open : Cell<bool>
}

impl RelpService {
    /// Creates a new `BytesCodec` for shipping around raw bytes.
    pub fn new() -> RelpService {
        RelpService{
            transaction_number: Cell::new(1),
            is_connection_open: Cell::new(false)
        }
    }
    fn new_for_test(tx : usize, is_open: bool) -> RelpService {
        RelpService{
            transaction_number: Cell::new(tx),
            is_connection_open: Cell::new(is_open)
        }
    }
}

impl Service for RelpService {
    // These types must match the corresponding protocol types:
    type Request = RelpSyslogMessage;
    type Response = RelpResponse;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = Box<Future<Item = RelpResponse, Error =  Self::Error>>;

    // Produce a future for computing a response from a request.
    fn call(&self, request: Self::Request) -> Self::Future {

        println!("Message:{}", request.transaction_number);
        io::stdout().flush().unwrap();

        if !self.is_connection_open.get() {
            if request.command != SyslogCommand::Open {
                Box::new(future::ok(RelpResponse{
                    transaction_number : 0,
                    status: ResponseStatusCode::ERROR,
                    message : Some("Cannot open connection that is already open".to_string())
                }))
            }
                //Could also check here that transaction number is 1, but lets assume it is
            else {
                self.is_connection_open.set(true);
                self.transaction_number.set(self.transaction_number.get() +1);

                Box::new(future::ok(RelpResponse{
                    transaction_number : self.transaction_number.get(),
                    status: ResponseStatusCode::OK,
                    message : Some("Cannot close connection that is not open".to_string())
                }))
            }
        } else if request.command == SyslogCommand::Close {
            if !self.is_connection_open.get() {

                Box::new(future::ok(RelpResponse{
                    transaction_number : 0,
                    status: ResponseStatusCode::ERROR,
                    message : Some("Cannot close connection that is not open".to_string())
                }))
            } else {
                self.is_connection_open.set(false);
                Box::new(future::ok(RelpResponse{
                    transaction_number : self.transaction_number.get(),
                    status: ResponseStatusCode::OK,
                    message : None
                }))
            }
        } else{
            if request.transaction_number != self.transaction_number.get() + 1 {
                //Close by default on error
                self.is_connection_open.set(false);

                Box::new(future::ok(RelpResponse{
                    transaction_number : self.transaction_number.get(),
                    status: ResponseStatusCode::ERROR,
                    message : None
                }))
            } else{
                //Do something here with the message
                self.transaction_number.set(self.transaction_number.get() +1);

                Box::new(future::ok(RelpResponse{
                    transaction_number : self.transaction_number.get(),
                    status: ResponseStatusCode::OK,
                    message : None
                }))
            }
        }
    }
}