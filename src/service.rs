use tokio_service::Service;
use futures::{future, Future};
use model::{ResponseStatusCode,RelpResponse, RelpSyslogMessage, SyslogCommand};
use std::io::{self, ErrorKind, Write, Error};
use std::cell::RefCell;
use std::rc::Rc;

pub struct RelpService{
    transaction_number : Rc<RefCell<usize>>,
    is_connection_open : Rc<RefCell<bool>>
}

impl RelpService {
    /// Creates a new `BytesCodec` for shipping around raw bytes.
    pub fn new() -> RelpService {
        RelpService{
            transaction_number: Rc::new(RefCell::new(1)),
            is_connection_open: Rc::new(RefCell::new(false))
        }
    }
    fn new_for_test(tx : usize, is_open: bool) -> RelpService {
        RelpService{
            transaction_number: Rc::new(RefCell::new(tx)),
            is_connection_open: Rc::new(RefCell::new(is_open))
        }
    }
}

//Follow this example https://github.com/tokio-rs/tokio-proto/blob/master/src/util/client_proxy.rs#L23

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

        let is_open_clone = self.is_connection_open.clone();
        let mut is_open = is_open_clone.borrow_mut();

        let tx_clone =  self.transaction_number.clone();
        let mut tx = tx_clone.borrow_mut();

        println!("Message tranaction number:{}", request.transaction_number);
        println!("Message command:{:?}", request.command);
        match request.data {
            Some(ref p) => println!("Message data:{}", p),
            None => println!("No message data"),
        }
        println!("Message data length:{}", request.data_length);


        println!("TX in state {}", *tx);
        println!("Is open in state {}", *is_open);

        if !(*is_open) {

            if request.command != SyslogCommand::Open {
                println!("Connection isn't open, did not recieve open command");
                io::stdout().flush().unwrap();

                Box::new(future::ok(RelpResponse{
                    transaction_number : 0,
                    status: ResponseStatusCode::ERROR,
                    message : Some("Cannot send open command when connection is already open".to_string())
                }))
            }
                //Could also check here that transaction number is 1, but lets assume it is
            else {
                println!("Connection isn't open, opening connection");
                io::stdout().flush().unwrap();

                *is_open = true;
                *tx += 1;

                Box::new(future::ok(RelpResponse{
                    transaction_number : request.transaction_number,
                    status: ResponseStatusCode::OK,
                    message : None
                }))
            }
        } else if request.command == SyslogCommand::Close {
            if !(*is_open) {
                println!("Cannot close connection that is not open");
                io::stdout().flush().unwrap();
                Box::new(future::ok(RelpResponse{
                    transaction_number : request.transaction_number,
                    status: ResponseStatusCode::ERROR,
                    message : Some("Cannot close connection that is not open".to_string())
                }))
            } else {
                println!("Closing connection");
                io::stdout().flush().unwrap();
                *is_open = false;
                Box::new(future::ok(RelpResponse{
                    transaction_number : request.transaction_number,
                    status: ResponseStatusCode::OK,
                    message : None
                }))
            }
        } else{
            if request.command == SyslogCommand::Open {
                println!("Connection is already open, cannot reopen");
                io::stdout().flush().unwrap();

                Box::new(future::ok(RelpResponse{
                    transaction_number : 0,
                    status: ResponseStatusCode::ERROR,
                    message : Some("Cannot send open command when connection is already open".to_string())
                }))
            }
            else if request.transaction_number != (*tx) {
                println!("Request transaction number {} does not match transaction number in state {}", request.transaction_number, *tx);
                io::stdout().flush().unwrap();
                //Close by default on error
                *is_open = false;

                Box::new(future::ok(RelpResponse{
                    transaction_number : request.transaction_number,
                    status: ResponseStatusCode::ERROR,
                    message : None
                }))
            } else{
                println!("Syslog message being processed for tx number {}", *tx);
                io::stdout().flush().unwrap();
                //Do something here with the message
                *tx += 1;
                Box::new(future::ok(RelpResponse{
                    transaction_number : request.transaction_number,
                    status: ResponseStatusCode::OK,
                    message : None
                }))
            }
        }
    }
}