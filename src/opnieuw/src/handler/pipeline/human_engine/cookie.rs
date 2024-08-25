use crate::tls::models::TlsFingerprint;
use crate::{debug, GA};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeCookie {
    pub ip: String,
    pub exp: u64,
    pub website: String,
    pub tls_fingerprint: TlsFingerprint, // that it was some OKAY browser
    pub ua: String,
}

impl ChallengeCookie {
    // MASSIVE TODO: env variables here!
    pub fn create_cookie(&self) -> String {
        GA.cookie.cookie_generated.inc();

        // debug!("cookie generated: {}", encode(&Header::default(), &self, &EncodingKey::from_secret("wuebfvjsdfnvbjlkefng8u324qtyhg".as_ref())).unwrap());

        return encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret("wuebfvjsdfnvbjlkefng8u324qtyhg".as_ref()),
        )
        .unwrap();
    }

    pub fn get_cookie(token: &str) -> Option<Self> {
        // debug!("Cookie attempted to be decrypted: {}, result: {:?}", token, decode::<Self>(token, &DecodingKey::from_secret("wuebfvjsdfnvbjlkefng8u324qtyhg".as_ref()), &Validation::default()));

        match decode::<Self>(
            token,
            &DecodingKey::from_secret("wuebfvjsdfnvbjlkefng8u324qtyhg".as_ref()),
            &Validation::default(),
        ) {
            Ok(resp) => Some(resp.claims),
            Err(_e) => None,
        }
    }
}

/*
impl RequestContext {
    pub fn check_cookie(&self, cookie: &ChallengeCookie) -> bool {
        self.ip == cookie.ip && self.domain == cookie.website
    }
}

 */

// OLD cookie code
/* // I LOVE COOKIE !!!
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_cookie(mut user_agent: String, ip: String, domain: String) -> String {
    // first thing we do is reverse the user agent
    user_agent = user_agent.chars().rev().collect::<String>();
    // now, let's build the cookie, shall we?
    // I was going to implement HTTP vs HTTPs but then I noticed how that could pose usability concerns! So no!
    let base_cookie: u64 = crc64::crc64(0, format!("23.133.104.{}AppleWebKit{}TBF{}696969", user_agent, ip, domain).as_bytes())
        + 2982709452689540142; // hash of "you should be working here"
    // I know they say don't unwrap in production ... but at this point, I would panic if it didn't work
    (((((((base_cookie + SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()) * 1294) + 48) * 1923) / 2000) + 69) * 23).to_string() // every server should have a custom "final" multiplication value
}

pub fn check_cookie(user_agent: String, ip: String, domain: String, cookie: &str) -> bool {
    if cookie.is_empty() {
        return false
    }
    let cookie: u64 = match cookie.parse::<u64>() { // shadow previous cookie value
        Ok(t) => t,
        Err(_) => { return false }
    };
    let base_cookie: u64 = crc64::crc64(0, format!("23.133.104.{}AppleWebKit{}TBF{}696969", user_agent, ip, domain).as_bytes())
        + 2982709452689540142; // hash of "you should be working here"
    // now, time to peel back the layers of the cookie
    let usr_checksum: u64 = (((((cookie / 23) - 69) * 2000) / 1923) - 48) / 1294;
    let time: u64 = usr_checksum - base_cookie; // get the time difference, this is in seconds since EPOCH
    if time > 86400 {
        return false
    }
    true
}

/*
struct Data4Cookie {
    pub user_agent: String,
    pub ip: String,
    pub domain: String // works for subdomains and main domain
}

impl Data4Cookie {
    pub fn cookie(&self) {

    }
}
 */
*/
