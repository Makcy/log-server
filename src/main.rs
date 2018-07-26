extern crate iron;
extern crate bodyparser;
extern crate router;

use iron::status;
use iron::prelude::*;
use router::{Router};
use std::io::prelude::*;
use std::io;
use std::fs::OpenOptions;

fn persistence_log(body: &[u8]) -> io::Result<()> {
    let filename = "file.log";
    let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .append(true)
                .open(filename);

    match file {
        Ok(mut stream) => {
            stream.write_all(body)?;
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
    Ok(())
}

fn log_body(req: &mut Request) -> IronResult<Response> {
    let body = req.get::<bodyparser::Raw>();
    match body {
        Ok(Some(body)) => {
            persistence_log(body.as_bytes()).ok();
        }, 
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }
    Ok(Response::with(status::Ok))
}

fn main() {
    let mut router = Router::new();
    router.post("/log", log_body, "log");
    println!("Listening on port 10080...");
    Iron::new(router).http("0.0.0.0:10080").unwrap();   
}
