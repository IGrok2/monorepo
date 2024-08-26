use aes::Aes128;

use crate::{
    debug,
    CHALLENGE_KEYS,
    GA,
};
use aes_gcm::{
    aead::{
        consts::U12,
        generic_array::GenericArray,
        Aead,
        AeadMut,
    },
    Aes128Gcm,
    AesGcm,
    KeyInit,
};
use base64::Engine;

impl ParsedChallengeResponse {
    pub fn decrypt(raw: Vec<u8>) -> Result<ParsedChallengeResponse, &'static str> {
        // convert the raw bytes from the body into a string
        let message = match String::from_utf8(raw.to_vec()) {
            Ok(t) => t,
            Err(_e) => {
                GA.cgi.challenge_not_utf8.inc();
                return Err("invalid");
            }
        };

        // take it out of base64 and back to bytes
        let raw_message = match base64::engine::general_purpose::STANDARD.decode(message) {
            Ok(t) => t,
            Err(_e) => {
                GA.cgi.challenge_bad_base64.inc();
                return Err("bad base64");
            }
        };

        // too short to have the required data
        if raw_message.len() < 28 {
            GA.cgi.challenge_too_short.inc();

            return Err("too short");
        }

        // init vector
        let nonce = GenericArray::from_slice(&raw_message[..12]);
        // actual encrypted data
        let ciphertext = &raw_message[12..raw_message.len()];

        // attempt to decrypt
        let decrypted = match CHALLENGE_KEYS.read().unwrap().decrypt(nonce, ciphertext) {
            Ok(t) => t,
            Err(_) => {
                GA.cgi.challenge_decryption_fail.inc();
                return Err("decryption error");
            }
        };

        // now we have the decrypted vector, let's try to parse it into a string
        let as_str = match String::from_utf8(decrypted) {
            Ok(t) => t,
            Err(_) => {
                GA.cgi.challenge_decryption_not_utf8.inc();
                return Err("decrypted not utf8");
            }
        };

        debug!("Challenge response as string: \n\n{}", as_str);

        let f: Temp = match serde_json::from_str(&as_str) {
            Ok(t) => t,
            Err(_e) => {
                debug!("unable to format: {:?}", _e);
                GA.cgi.challenge_unable_to_format.inc();
                return Err("unable to format challenge");
            }
        };

        Ok(ParsedChallengeResponse {
            browser: f.goldbear,
            user_agent_browser: f.fake,
            epoch: f.backspace,
            language: f.energy,
            video_card: f.eloquent,
            window_height: f.homies,
            window_width: f.manifest,
            plugins: f.joypad,
            notifications_persistent: f.annoyed,
            fixed_memory_set: f.diamond,
            audio_context: false,
        })
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Temp {
    pub goldbear: String,
    pub fake: String,
    pub backspace: u64,

    // statistics
    pub energy: String,
    pub eloquent: String,
    pub homies: u64,
    pub manifest: u64,
    pub joypad: u64,

    // beta checks
    pub annoyed: bool,
    pub diamond: bool, // from baloo
}

#[derive(serde::Deserialize, Debug)]
pub struct ParsedChallengeResponse {
    pub browser: String,
    pub user_agent_browser: String,
    pub epoch: u64,

    // statistics
    pub language: String,
    pub video_card: String,
    pub window_height: u64,
    pub window_width: u64,
    pub plugins: u64,

    // beta checks
    pub notifications_persistent: bool,
    pub fixed_memory_set: bool, // from baloo
    pub audio_context: bool,    // from baloo (audio context sample rate)
}
