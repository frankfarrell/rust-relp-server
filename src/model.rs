use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SyslogCommand {
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ResponseStatusCode {
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

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RelpSyslogMessage {
    //Needs to be between 1 and 999_999_999
    pub transaction_number: usize,
    pub command : SyslogCommand,
    /*
    Number of bytes in data
    Needs to be between 1 and 999_999_999
    Typically only up to 128KB?
     */
    pub data_length: usize,
    //Can this be empty? Eg if data length is zero?
    pub  data: Option<String>
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RelpResponse {
    //Needs to be between 1 and 999_999_999
    pub transaction_number: usize,
    pub status : ResponseStatusCode,
    pub  message: Option<String>
}