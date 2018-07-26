extern crate iron;
extern crate bodyparser;
extern crate router;
extern crate chrono;

use iron::status;
use iron::prelude::*;
use router::{Router};
use std::io::prelude::*;
use std::io;
use std::fs::OpenOptions;
use std::path::Path;
use chrono::Local;

fn persistence_log(body: &[u8]) -> io::Result<()> {
    let date = Local::now().format("%Y-%m-%d");
    let file_path = "/data/logServer/".to_string() + &(date.to_string() + "-kong.log");
    let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .append(true)
                .open(Path::new(&file_path));

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
    println!("Listening on port 3000...");
    Iron::new(router).http("0.0.0.0:3000").unwrap();   
}
