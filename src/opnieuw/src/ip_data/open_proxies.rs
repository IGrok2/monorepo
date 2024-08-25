use std::fs::File;
use std::io::Read; // need this for .read_to_string
use std::net::Ipv4Addr;
use std::process::Command;
use std::str::FromStr;

pub fn get_open_proxies() -> Vec<Ipv4Addr> {
    Command::new("curl") // TODO: make this directory
        .args([
            "https://api.iprisk.info/open-proxies-v4",
            "-o",
            "proxies.txt",
        ])
        .output()
        .expect("error running proxy command");

    let mut proxies = vec![];

    let mut data = String::new();
    File::open("proxies.txt")
        .expect("couldn't open file")
        .read_to_string(&mut data)
        .expect("couldn't read file to string");

    let new_data = data.split("\n");

    for i in new_data {
        // when we have it in port notation
        // let spl: Vec<&str> = i.split(":").collect();
        // if let Ok(t) = Ipv4Addr::from_str(spl.get(0).unwrap()) {
        if let Ok(t) = Ipv4Addr::from_str(i) {
            proxies.push(t)
        }
    }

    proxies
}
