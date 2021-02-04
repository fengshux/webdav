#[macro_use]
extern crate serde_derive;
extern crate toml;
#[macro_use]
extern crate log;
extern crate use simple_logger;

mod webdav;

use simple_logger::SimpleLogger;
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
    SimpleLogger::new().init().unwrap();
    let config = init_config();

    println!("======================= webdav ===========================");
    let dav = Webdav::new("https://dav.jianguoyun.com/dav/schedule/", config.webdav);
    // dav.write("schedule.org");
    let res = dav.list();
    println!("{:?}", res);

    // println!("======================= local ===========================");
    // let native = Native::new("schedule");
    // let locals = native.list();
    // println!("{:?}", locals);
    
}
