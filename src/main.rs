extern crate hyper;
extern crate url;
extern crate rustc_serialize;

use rustc_serialize::json::{self, Json};

use hyper::client::Client;
use hyper::header::{ContentType, Connection};
use hyper::error::Error;
use std::io::Read;
use url::Url;
use hyper::status::StatusCode;


pub struct HttpResponse {
    pub status: StatusCode,
    pub body: String
}

fn main() {
    let url: Url = Url::parse("http://localho3st:8983/solr/gettingstarted/select?q=foundation&wt=json&indent=true").unwrap();

    let r = get(&url);
    match r {
    	Ok(response) => {
    		println!("{}", response.body)
    	},
    	Err(e) => println!("Some error occured: {:?}", e)
    }
}

pub fn get(url: &Url) -> Result<HttpResponse, Error> {
	let mut client = Client::new();
   	let mut result_response = client.get(&url.to_string()).send();
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
