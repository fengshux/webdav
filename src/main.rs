#[macro_use]
extern crate serde_derive;
extern crate toml;
#[macro_use]
extern crate log; 

use reqwest::Method;
use serde::Deserialize;
use quick_xml::de::{from_str, DeError};
use std::fs;
use chrono::prelude::*;

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
    debug!("{:?}", config);
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
    fn list(&self) -> Box<Vec<Filestatus>> {
        let url = &self.path;
        let client = reqwest::blocking::Client::new();
        let body = client
            .request(Method::from_bytes(b"PROPFIND").unwrap(), url)
            .basic_auth(&self.account.username, Some(&self.account.password))
            .send()
            .unwrap()
            .text()
            .unwrap();
        debug!("{}", body);
        let multistatus: Multistatus = from_str(&body).unwrap();
        let mut files: Vec<Filestatus> = Vec::new();

        for response in multistatus.response {
            // rfc1123
            //Sat, 02 Jan 2021 04:59:35 GMT
            //%a, %d %b %Y %T %Z
            let modify = DateTime::parse_from_str(&response.propstat.prop.getlastmodified, "%a, %d %b %Y %T %Z");
            if let Ok(modify_time) = modify {
                let f = Filestatus{
                    path: response.href,
                    lastmodified: modify_time,
                    contentlength: response.propstat.prop.getcontentlength,                
                    owner: response.propstat.prop.owner,
                    contenttype:response.propstat.prop.getcontenttype,
                    name:response.propstat.prop.displayname,
                };
                files.push(f)                
            } else {
               panic!("covert date type failur")
            }
        }
        Box::new(files)
    }
}

#[derive(Debug)]
struct Filestatus {
    path: String,
    lastmodified: DateTime<FixedOffset>,
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
struct Prop{
    getlastmodified: String,
    getcontentlength: i64,
    owner: String,
    getcontenttype: String,
    displayname: String,
}

struct Native {
    path: String
}

impl Native {
    fn list(self) {
        match fs::read_dir(self.path) {
            Ok(dir) => {
                for entry in dir {
                    if let Ok(entry) = entry {
                        // Here, `entry` is a `DirEntry`.
                        if let Ok(metadata) = entry.metadata() {
                            // Now let's show our entry's permissions!
                            if let Ok(modified) = metadata.modified() {
                                println!("{:?}: {:?}", entry.path(), modified);
                            } else {
                                println!("{:?} Could not get modified", entry.path());
                            }
                            
                        } else {
                            println!("Couldn't get metadata for {:?}", entry.path());
                        }
                    }
                }                                   
            },
            Err(e)  => println!("{}",e),
        }
    }
}



fn main() {
   let config = init_config();

    println!("======================= webdav ===========================");
    let dav = Webdav::new("https://dav.jianguoyun.com/dav/schedule/", config.webdav);
    let res = dav.list();
    println!("{:?}", res);

    println!("======================= local ===========================");
    let local = Native{path:"schedule".to_string()};
    local.list();
}
