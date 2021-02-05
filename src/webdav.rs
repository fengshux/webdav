use reqwest::{Method, header};
use quick_xml::de::{from_str};
use std::fs;
use chrono::prelude::*;


#[derive(Debug)]
pub struct Filestatus {
    path: String,
    lastmodified: DateTime<Local>,
    contentlength: u64,
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
    getcontentlength: u64,
    owner: String,
    getcontenttype: String,
    displayname: String,
}


#[derive(Deserialize, Debug)]
pub struct Account {
    username: String,
    password: String,
}

pub struct Webdav {
    path: String,
    account: Account,
}

// webdav 代表的是remote的实体
impl Webdav {
    pub fn new(path: &str, account: Account) -> Self {
        Webdav {
            path: path.to_string(),
            account: account,
        }
    }
    pub fn list(&self) -> Box<Vec<Filestatus>> {
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
        let mut files: Vec<Filestatus> = Vec::new();

        for response in multistatus.response {

            let modify_time = DateTime::parse_from_rfc2822(&response.propstat.prop.getlastmodified).unwrap();
            let f = Filestatus{
                path: response.href,
                lastmodified: modify_time.with_timezone(&Local),
                contentlength: response.propstat.prop.getcontentlength,                
                owner: response.propstat.prop.owner,
                contenttype:response.propstat.prop.getcontenttype,
                name:response.propstat.prop.displayname,
            };
            files.push(f)                

        }
        Box::new(files)
    }   

    pub fn write(&self, file_name: &str) -> Result<(), String> {
        let url = &self.path;
        let client = reqwest::blocking::Client::new();
        let body = client
            .request(Method::PUT, &(url.to_string() + file_name))
            .basic_auth(&self.account.username, Some(&self.account.password))
            .body("the exact body that is sent")
            .send()
            .unwrap()
            .text()
            .unwrap();
        println!("{}",body);
        Ok(())
    }

    pub fn pro_patch(&self, file_name: String, prop: String, value: String ) -> Result<(), String> {

        let content =r#"<?xml version="1.0"?>
                        <d:propertyupdate xmlns:d="DAV:" xmlns:o="urn:schemas-microsoft-com:office:office">
                          <d:set>
                            <d:prop>
                              <o:Author>Douglas Groncki</o:Author>
                            </d:prop>
                          </d:set>
                        </d:propertyupdate>"#;

        
        let url = &self.path;
        let client = reqwest::blocking::Client::new();
        let mut headers = header::HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "text/xml".parse().unwrap());
        let body = client
            .request(Method::from_bytes(b"PROPPATCH").unwrap(), url)
            .basic_auth(&self.account.username, Some(&self.account.password))
            .headers(headers)
            .body("the exact body that is sent")
            .send()
            .unwrap()
            .text()
            .unwrap();
        println!("{}", body);
        
        Ok(())
    }
}


pub struct Native {
    path: String
}

impl Native {
    pub fn new(path: &str) -> Self {
        Native{
            path: path.to_string()
        }
    }    
    pub fn list(self) -> Box<Vec<Filestatus>> {
        let mut files: Vec<Filestatus> = Vec::new();
        match fs::read_dir(self.path) {
            Ok(dir) => {                
                for entry in dir {
                    if let Ok(entry) = entry {
                        // Here, `entry` is a `DirEntry`.
                        if let Ok(metadata) = entry.metadata() {
                            // Now let's show our entry's permissions!
                            let f = Filestatus{
                                path: entry.path().to_str().unwrap().to_string(),
                                lastmodified: DateTime::from(metadata.modified().unwrap()),
                                contentlength: metadata.len(),
                                owner: "".to_string(),
                                contenttype: "".to_string(),
                                name: entry.file_name().into_string().unwrap(),
                            };

                            files.push(f);
                            
                        } else {
                            println!("Couldn't get metadata for {:?}", entry.path());
                        }
                    }

                }
            },
            Err(e)  => println!("{}",e),
        }

        Box::new(files)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pro_patch( ) {
        assert!(true, "pro_patch works  incorrect!");
    }
}
