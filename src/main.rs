#[macro_use]
extern crate clap;
extern crate crypto;

use std::io::prelude::*;
use std::fs::OpenOptions;

use crypto::md5::Md5;
use crypto::digest::Digest;


fn main() {
    let matches = clap_app!(htpasswd =>
        (version: "1.0")
        (author: "Baptiste <baptiste@darthenay.fr>")
        (about: "Generates .htpasswd for Apache or Nginx")
        (@arg USERNAME: +required "Username")
        (@arg PASSWORD: +required "Password")
    ).get_matches();

    let username = matches.value_of("USERNAME").unwrap();
    let password = matches.value_of("PASSWORD").unwrap();
    let mut sh = Md5::new();
    sh.input_str(password);
    
    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(".htpasswd") {
        Err(why) => panic!("Couldn't create file"),
        Ok(file) => file,
    };
    
    match write!(file, "{}:{}\n", username, sh.result_str()) {
        Err(why) => panic!("Couldn't write file"),
        Ok(_) => println!("success"),
    };
    println!("{}:{}", username, sh.result_str());
}
