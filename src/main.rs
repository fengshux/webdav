#[macro_use]
extern crate serde_derive;
extern crate toml;
#[macro_use]
extern crate log;


mod webdav;


use std::fs;
use webdav::{Webdav, Native, Account};


#[derive(Deserialize, Debug)]
struct Conf {
    webdav: Account,
}

fn init_config() -> Conf {
    let config_path = "config.toml";
    let contents = fs::read_to_string(config_path).expect("Something went wrong reading the file");
    let config: Conf = toml::from_str(&contents).unwrap();
    debug!("{:?}", config);
    config
}

fn main() {
   let config = init_config();

    println!("======================= webdav ===========================");
    let dav = Webdav::new("https://dav.jianguoyun.com/dav/schedule/", config.webdav);
    let res = dav.list();
    println!("{:?}", res);

    println!("======================= local ===========================");
    let native = Native::new("schedule");
    let locals = native.list();
    println!("{:?}", locals);
}
