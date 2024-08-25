use crate::ip_data::models::IPAsnCountry;
use crate::ip_data::models::InitData;
use crate::utils::counter::Counter;
use std::fs::File;
use std::io::Read;
use std::net::Ipv4Addr;
use std::process::Command;
use std::str::FromStr;

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

    let new_data = data.split("\n");

    let mut data = vec![]; // shadow previous data value

    for i in new_data {
        let json = match serde_json::from_str::<InitData>(i) {
            Ok(t) => t,
            Err(_) => break,
        };
        match Ipv4Addr::from_str(&json.start_ip) {
            Ok(t) => {
                data.push(IPAsnCountry {
                    starting_ip: t,
                    ending_ip: Ipv4Addr::from_str(&json.end_ip).unwrap(), // this really shouldn't cause an error
                    asn: json.asn,
                    continent: json.continent,
                    country: json.country,
                    hits: Counter::new(),
                })
            }
            Err(_) => {} // skip it, likely ipv6
        }
    }

    data
}
