extern crate futures;
extern crate hyper;
extern crate serde_json;
extern crate hyper_tls;
extern crate tokio_core;

use std::io::{self, Read};
use futures::{Future, Stream};
use hyper::{Client, Method};
use hyper::client::Request;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use serde_json::Value;
use std::process::Command;

fn main() {
    
    let mut input = String::new();
    let content = match io::stdin().read_to_string(&mut input) {
        Ok(_) => input,
        Err(e) => panic!("Error: {}", e),
    };
    
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    let uri = "https://www.hastebin.com/documents".parse().unwrap();

    let mut req = Request::new(Method::Post, uri);
    req.set_body(content);

    let post = client.request(req).and_then(|res| {
        res.body().concat2().and_then(move |body| {
            let v: Value = match serde_json::from_slice(&body) {
                Ok(body) => body,
                Err(e) => panic!("{}", e)
            };
            Ok(v)
        })
    });

    match core.run(post) {
        Ok(v) => {
            let hastebin = format!("https://hastebin.com/{}", v["key"].to_string().replace("\"", ""));
            println!("👌  Uploaded on hastebin at {}", hastebin);
            Command::new("open").arg(hastebin).spawn().expect("Failed to open in your browser.");
        },
        Err(e) => panic!("An error occured : {}", e)
    }
}
