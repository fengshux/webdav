#[macro_use]
extern crate serde_derive;
extern crate toml;

use reqwest::Method;
use serde::Deserialize;
use quick_xml::de::{from_str, DeError};
use std::fs;

#[derive(Deserialize, Debug)]
struct Conf {
    webdav: Account,
}

#[derive(Deserialize, Debug)]
struct Account {
    username: String,
    password: String,
}

fn init_config() -> Conf {
    let config_path = "config.toml";
    let contents = fs::read_to_string(config_path).expect("Something went wrong reading the file");
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
        Webdav {
            path: path.to_string(),
            account: account,
        }
    }

    fn list(&self) -> Box<Vec<Davfile>> {
        let url = &self.path;
        let client = reqwest::blocking::Client::new();
        let body = client
            .request(Method::from_bytes(b"PROPFIND").unwrap(), url)
            .basic_auth(&self.account.username, Some(&self.account.password))
            .send()
            .unwrap()
            .text()
            .unwrap();
        println!("{}", body);
        let multistatus: Multistatus = from_str(&body).unwrap();
        let mut files: Vec<Davfile> = Vec::new();

        for response in multistatus {
            files.
        }

        Box::new(files)
    }
}

struct Davfile {
    path: String,
    lastmodified: String,
    contentlength: i64,
    owner: String,
    contenttype: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Multistatus {
    response: Vec<Response>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    href: String,
    propstat: Propstat,
}

#[derive(Debug, Serialize, Deserialize)]
struct Propstat {
    prop: Prop,
    status: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Prop {
    getlastmodified: String,
    getcontentlength: i64,
    owner: String,
    getcontenttype: String,
    displayname: String,
}


fn xml() {
   
    let contents = fs::read_to_string("test.xml").expect("Something went wrong reading the file");
    println!("{}", contents);
    let multistatus: Multistatus = from_str(&contents).unwrap();
    println!("{:?}", multistatus);
}

fn main() {
    // let config = init_config();
    // let dav = Webdav::new("https://dav.jianguoyun.com/dav/schedule/", config.webdav);
    // dav.list();

    xml();
}
