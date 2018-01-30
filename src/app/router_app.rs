use super::*;

pub struct RouterApp {
    map: HashMap<String, Box<App>>,
}

impl RouterApp {
    pub fn new() -> RouterApp {
        RouterApp {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, app: Box<App>) {
        self.map.insert(String::from(name),app);
    }
}

impl App for RouterApp {
    fn run(&mut self, mut url: Vec<&str>, data: &Vec<u8>) -> AppResult {
        println!("/{}", url[0]);
        let name = url.pop().unwrap();
        println!("/{}", name);
        match self.map.get_mut(name) {
            Some(app) => app.run(url, data),
            None => Ok(
                Response::from_string(format!("{} not found", name)
                ).with_status_code(tiny_http::StatusCode(404)))
        }
    }
}