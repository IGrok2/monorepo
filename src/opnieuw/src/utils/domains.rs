use crate::TLDS;
use hyper::Request;
use std::{
    fs::File,
    io::Read,
    process::Command,
    rc::Rc,
};

pub fn is_domain(host: &str) -> bool {
    let split_up: Vec<&str> = host.split('.').collect();
    for i in TLDS.iter() {
        if i == &split_up[split_up.len() - 1].to_lowercase() {
            return true; // then it's actually a domain
        }
    }
    false
}

pub fn get_user_agent(req: &Request<hyper::body::Incoming>) -> Option<String> {
    match req.headers().get("User-Agent") {
        Some(t) => match t.to_str() {
            Ok(t) => Some(t.to_string()),
            Err(_) => None,
        },
        None => None,
    }
}

pub fn get_tld_list() -> Vec<String> {
    Command::new("curl") // TODO: make this directory
        .args([
            "https://data.iana.org/TLD/tlds-alpha-by-domain.txt",
            "-o",
            "tlds.txt",
        ])
        .output()
        .expect("error running tld command");

    let mut tlds = vec![];

    let mut data = String::new();
    File::open("tlds.txt")
        .expect("couldn't open file")
        .read_to_string(&mut data)
        .expect("couldn't read file to string");

    let new_data = data.split('\n');

    for i in new_data {
        // get each newline
        if !i.contains('#') {
            // ignore comments
            tlds.push(i.to_lowercase())
        }
    }

    tlds
}
