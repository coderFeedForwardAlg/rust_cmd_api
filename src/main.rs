use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

use std::fs::File;
use std::process::Command;

use rocket::State;




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


#[post("/edit/<file>", format = "application/json", data = "<text>")]
fn comm2(file: &str, text: &str) -> String {
    // if command == "cd" {) -> String {
    // println!("{} ", Dir::dir);
    // if command == "cd" {
    //     dir = dir + args;
    //     return "moved dir";
    // }
    // let arr: Vec<&str> = args.split("+").collect();
    /*let p1 = Command::new("echo")
        .current_dir("./to_run")
        .args([text, ">", file])
        .output(); */
        // .expect("failed to execute process");
    // println!("status: {}", output.unwrap().status);

    // let res = output.unwrap()

    //let hello = String::from_utf8(p1.unwrap().stdout).unwrap(); //res.stdout;

    let mut file = std::fs::File::create(file);
    file.expect("reason").write_all(text.as_bytes());
    return "saved".to_string();

}

// why use a struct wit hone thing ??? fix
struct Dir {
    dir: String
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![comm, comm2]).manage(Dir { dir: "hello".to_string() })
}




