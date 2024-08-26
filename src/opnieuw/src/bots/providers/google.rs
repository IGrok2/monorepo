use ip_network::Ipv4Network;
use serde_json::Value;
use std::str::FromStr;

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize)]
struct GoogleResp {
    creationTime: String, // going to have to ignore this warning because this is the way it's placed in the JSON
    prefixes: Vec<Value>,
}

pub async fn get_google() -> Vec<String> {
    // gets google's IPs
    // google's IPs that we will end up returning
    let mut ips: Vec<String> = Vec::new();
    // send the request to fetch IPs
    let req =
        reqwest::get("https://developers.google.com/static/search/apis/ipranges/googlebot.json")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

    let prefixes = serde_json::from_str::<GoogleResp>(req.as_str())
        .unwrap()
        .prefixes;

    for val in prefixes.iter() {
        if let Some(t) = val.get("ipv4Prefix") {
            let block = t.as_str().unwrap();
            let net = Ipv4Network::from_str(block).unwrap();
            for val in net.hosts() {
                ips.push(val.to_string())
            }
        }
    }

    ips
}
