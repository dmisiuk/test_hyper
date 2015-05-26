extern crate hyper;
extern crate url;
extern crate serde;
extern crate rustc_serialize;

use rustc_serialize::json as rjson;

use hyper::client::Client;
use hyper::header::ContentType;
use hyper::error::Error;
use std::io::Read;
use url::Url;
use hyper::status::StatusCode;
use serde::json::{self,Value};

pub struct HttpResponse {
    pub status: StatusCode,
    pub body: String
}

fn main() {
    let url: Url = Url::parse("http://localhost:8983/solr/gettingstarted/select?q=*:*&wt=json&indent=true").unwrap();

    let url_post = Url::parse("http://localhost:8983/solr/gettingstarted/update").unwrap();
    let r = get(&url);
    match r {
        Ok(response) => {
            //println!("{}", response.body);
            let json_resp: Value= json::from_str(&response.body).unwrap();
            let docs = json_resp.as_object().unwrap().get("response").unwrap().as_object().unwrap().get("docs").unwrap();
            for doc in docs.as_array().unwrap() {
                println!("{:?}", doc);
            }

        },
        Err(e) => println!("Some error occured: {:?}", e)
    }

    let r_post = post(&url_post);

    match r_post {
        Ok(response) => {
            //println!("{}", response.body);
            let json_resp = rjson::Json::from_str(&response.body).unwrap();
            println!("{}", json_resp);
            println!("Debug: {:?}", json_resp.as_object().unwrap());
        },
        Err(e) => println!("Some error occured: {:?}", e)
    }
}

pub fn get(url: &Url) -> Result<HttpResponse, Error> {
    let mut client = Client::new();
    let result_response = client.get(&url.to_string()).send();
    match result_response {
        Ok(mut res) => {
            let mut body = String::new();
            let result = res.read_to_string(&mut body);
            match result {
                Ok(_) => {
                    Ok(HttpResponse{status: res.status, body: body})
                },
                Err(err) => {
                    Err(Error::Io(err))
                }
            }
        },
        Err(err) => Err(err)
    }
}


pub fn post(url: &Url) -> Result<HttpResponse, Error> {
    let mut client = Client::new();
    let result_response = client.post(&url.to_string())
        .header(ContentType::json())
        .body("{add: {doc: {id: \"7\", title: \"petia2\", v: {inc: 1}}, boost: 1, overwrite: true, commitWithin: 1000}}")
        .send();
    match result_response {
        Ok(mut res) => {
            let mut body = String::new();
            let result = res.read_to_string(&mut body);
            match result {
                Ok(_) => {
                    Ok(HttpResponse{status: res.status, body: body})
                },
                Err(err) => {
                    Err(Error::Io(err))
                }
            }
        },
        Err(err) => Err(err)
    }
}
