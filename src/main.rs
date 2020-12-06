#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::fs;
use reqwest::Method;

#[derive(Deserialize)]
#[derive(Debug)]
struct Conf {
    webdav: Account,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct Account {
    username: String,
    password: String,
}


fn init_config() -> Conf {
    let config_path = "config.toml";
    let contents = fs::read_to_string(config_path)
        .expect("Something went wrong reading the file");
    let config: Conf = toml::from_str(&contents).unwrap();    
    println!("{:?}", config);
    config
}


struct Webdav {
    path: String,
    account: Account,    
}

impl Webdav {
    fn new(path: &str, account: Account) -> Self {
        Webdav{path:path.to_string(), account:account}
    }

    fn list(&self) {

        let url = &self.path;
        let client = reqwest::blocking::Client::new();
        let body = client.request(Method::from_bytes(b"PROPFIND").unwrap(),url).basic_auth(&self.account.username, Some(&self.account.password))
            .send().unwrap().text().unwrap();
        println!("{}", body);
    }
}



fn main() {
    let config = init_config();
    let dav = Webdav::new("https://dav.jianguoyun.com/dav/schedule/", config.webdav);
    dav.list();
}



