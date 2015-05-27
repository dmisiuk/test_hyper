#![crate_name="test_hyper"]

extern crate hyper;
extern crate url;
extern crate serde;
extern crate rustc_serialize;



pub mod test {


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
}