use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs; 

use std::process::Command;
use std::io::{self, Write};


#[macro_use] extern crate rocket;

// fn main() {


//     let output = Command::new("cargo")
//         .current_dir("./to_run")
//         .args(["run"])
//         .output();
//         // .expect("failed to execute process");
//     // println!("status: {}", output.unwrap().status);

//     let res = output.unwrap();

//     io::stdout().write_all(&res.stdout).unwrap();

// }
#[get("/cmd/<command>")]
fn comm(command: &str) -> String {
    let p1 = Command::new(command)
        // .current_dir("./to_run")
        // .args(["run"])
        .output();
        // .expect("failed to execute process");
    // println!("status: {}", output.unwrap().status);

    // let res = output.unwrap()
    let hello = String::from_utf8(p1.unwrap().stdout).unwrap(); //res.stdout;
    return hello;


    
}



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![comm])
}


