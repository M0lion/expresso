use app::App;
use super::tiny_http;
use super::std::error::Error;

pub struct Server<'a> {
    server: tiny_http::Server,
    pub root: &'a mut App
}

impl<'a> Server<'a> {
    pub fn from_app(root: &'a mut App, host: &str) -> Result<Server<'a>, Box<Error + Send + Sync + 'static>> {
        match tiny_http::Server::http(host) {
            Ok(server) => {
                println!("server started at {}", server.server_addr());
                Ok(Server {
                    server,
                    root
                })
            },
            Err(e) => Err(e)
        }
    }

    /*pub fn new(host: &str) -> Result<Server<'a>, Box<Error + Send + Sync + 'static>> {
        match tiny_http::Server::http("0.0.0.0:8000") {
            Ok(server) => {
                println!("server started at {}", server.server_addr());
                let root = &mut app::router_app::RouterApp::new();
                Ok(Server {
                    server,
                    root
                })
            },
            Err(e) => Err(e)
        }
    }*/

    pub fn run(&mut self) {
        for mut request in self.server.incoming_requests() {
            println!("received request!\nmethod: {:?}\nurl: {:?}",
                request.method(),
                request.url(),
            );

            let url = String::from(request.url());
            let mut url = url.split('/').collect::<Vec<&str>>();
            url.pop().unwrap();

            let mut data = Vec::new();
            request.as_reader().read_to_end(&mut data).unwrap();

            match self.root.run(url, &data) {
                Ok(resp) => request.respond(resp),
                Err(err) => {
                    let err_str = format!("{}", err);
                    println!("{}", err_str);
                    request.respond(tiny_http::Response::from_string(err_str).with_status_code(tiny_http::StatusCode(400)))
                }
            }.unwrap();
        }
    }
}