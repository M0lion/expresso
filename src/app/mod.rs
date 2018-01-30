use tiny_http;
use tiny_http::Response;
use ::std::collections::HashMap;
use ::std::io::Cursor;
use ::std::vec::Vec;
use ::std::error::Error;

pub type AppResult = Result<Response<Cursor<Vec<u8>>>, AppError>;

pub mod router_app;

pub trait App {
    fn run(&mut self, url: Vec<&str>, data: &Vec<u8>) -> AppResult;
}

pub struct AppError {
    msg: String,
}

impl AppError {
    pub fn new(msg: String) -> AppError {
        AppError {
            msg
        }
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        &(self.msg)
    }
}

impl ::std::fmt::Display for AppError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl ::std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "AppError({})", self.msg)
    }
}