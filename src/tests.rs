use serde_json;
use tiny_http;

use ::std::vec::Vec;
use ::app::{App,AppResult};
use ::app::router_app::RouterApp;

use ::server;

#[test]
fn it_works() {
    let test: serde_json::Value = serde_json::from_str("{ \"test\": 5, \"lol\": [1, 2, 3] }").unwrap();
    println!("tesT: {:?}", test);

    let mut app = RouterApp::new();
    app.add(&"test", Box::new(MyApp {
        data: "test"
    }));

    app.add(&"lol", Box::new(MyApp::from_str("lol")));

    let mut server = server::Server::from_app(& mut app, &"0.0.0.0:8000").unwrap();

    server.run();
}


struct MyApp<'a> {
    data: &'a str,
}

impl<'a> MyApp<'a> {
    fn from_str(s: &str) -> MyApp {
        MyApp {
            data: s
        }
    }
}

impl<'a> App for MyApp<'a> {
    fn run(&mut self, _url: Vec<&str>, _data: &Vec<u8>) -> AppResult {
        Ok(tiny_http::Response::from_string(self.data))
    }
}