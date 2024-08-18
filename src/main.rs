use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs; 

use std::process::Command;




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


// let mut dir = "./";
#[get("/cmd/<command>/<args>")]
fn comm(command: &str, args: &str) -> String {
    // if command == "cd" {
    //     dir = dir + args;
    //     return "moved dir";
    // } 
    let arr: Vec<&str> = args.split("+").collect();
    let p1 = Command::new(command)
        .current_dir("./to_run/src")
        .args(arr)
        .output();
        // .expect("failed to execute process");
    // println!("status: {}", output.unwrap().status);

    // let res = output.unwrap()
    
    let hello = String::from_utf8(p1.unwrap().stdout).unwrap(); //res.stdout;
    return hello;

}

#[get("/cmd/<command>/<args>")]
fn comm2(command: &str, args: &str) -> String {
    // if command == "cd" {
    //     dir = dir + args;
    //     return "moved dir";
    // } 
    let arr: Vec<&str> = args.split("+").collect();
    let p1 = Command::new(command)
        .current_dir("./to_run")
        .args(arr)
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


