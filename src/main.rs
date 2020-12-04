#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::fs;

#[derive(Deserialize)]
#[derive(Debug)]
struct Conf {
    webdav: Webdav,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct Webdav {
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

fn main() {
    let config = init_config();
    let url = "https://dav.jianguoyun.com/dav/schedule";
    let client = reqwest::blocking::Client::new();
    let body = clent.request("PROPFIND",url).unwrap()
        .text().unwrap();
    println!("{}", body);
}
