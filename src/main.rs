use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

use rocket::http::Method;

use std::process::Command;

use rocket::State;
use rocket::tokio::fs::{self, File};

use std::fs::OpenOptions;
use std::io::{BufReader, BufRead, Write}; 

// use export ROCKET_ADDRESS=0.0.0.0 to set env befor starting server if you havent 

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



//ray_env/bin/python3 run_cmd/file.py

#[post("/run_ray", format = "multipart/form-data", data = "<text>")]
async fn run_ray(text: &str) -> String {
    let mut file = std::fs::File::create("file.py");
    file.expect("reason").write_all(text.as_bytes());


    let mut file = OpenOptions::new()
    .read(true)
    .write(true)
    .open("file.py")
    .expect("file.txt doesn't exist or so");


    let mut lines = BufReader::new(file).lines().skip(3)
    .map(|x| x.unwrap())
    .collect::<Vec<String>>().split_last().unwrap().1.join("\n");

    fs::write("file.py", lines).await.expect("Can't write");

    println!("writing file to disk");

    let p1 = Command::new("../../ray_env/bin/python3").arg("file.py").output();
    
    let res = String::from_utf8(p1.unwrap().stdout).unwrap();
    println!("{}", res);
    return res.to_string(); // res.to_string();

}




// why use a struct wit hone thing ??? fix
struct Dir {
    dir: String
}


fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::all();

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error creating CORS")
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![comm, comm2, run_ray]).attach(make_cors())
}




