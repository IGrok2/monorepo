use crate::{
    ip_data::models::{
        IPAsnCountry,
        InitData,
    },
    utils::counter::Counter,
};
use std::{
    fs::File,
    io::Read,
    net::Ipv4Addr,
    process::Command,
    str::FromStr,
};

pub fn get_ip_data() -> Vec<IPAsnCountry> {
    Command::new("curl") // TODO: make this directory
        .args([
            "-L",
            "https://ipinfo.io/data/free/country_asn.json.gz?token=3e0d8bb6691bc7",
            "-o",
            "ip_data.json.gz",
        ])
        .output()
        .expect("error running ip data cmd");

    Command::new("gzip")
        .args(["-d", "ip_data.json.gz"])
        .output()
        .expect("error decompressing ip data");

    let mut data = String::new();
    File::open("ip_data.json")
        .expect("couldn't open file")
        .read_to_string(&mut data)
        .expect("couldn't read file to string");

    let new_data = data.split('\n');

    let mut data = vec![]; // shadow previous data value

    for i in new_data {
        let json = match serde_json::from_str::<InitData>(i) {
            Ok(t) => t,
            Err(_) => break,
        };
        if let Ok(starting_ip) = Ipv4Addr::from_str(&json.start_ip) {
            data.push(IPAsnCountry {
                starting_ip,
                ending_ip: Ipv4Addr::from_str(&json.end_ip).unwrap(), // this really shouldn't cause an error
                asn: json.asn,
                continent: json.continent,
                country: json.country,
                hits: Counter::new(),
            })
        }
    }

    data
}
