use std::io::prelude::*;

use std::io::BufReader;
use std::path::Path;
use std::fs::File;

use std::ffi::{CStr};
use std::os::raw::c_char;

use rand::{self};

use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signer};
use rand::rngs::OsRng;
use toml::{self};

use serde::{de, Serialize, Deserialize};

use log::{info, warn, error};



pub const SMCU_OK: i32 = 0;

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(deserialize_with = "deserialize_b58_pub")]
    pub_key: PublicKey,

    #[serde(deserialize_with = "deserialize_b58_priv")]
    priv_key: SecretKey,
}

fn deserialize_b58_pub<'de, D>(deserializer: D) -> Result<PublicKey, D::Error>
where D: de::Deserializer<'de> {
    let buf = String::deserialize(deserializer)?;
    let mut data = [0u8; ed25519_dalek::PUBLIC_KEY_LENGTH];
    let _ = bs58::decode(buf).into(&mut data).map_err(|e| de::Error::custom(format!("base58 decode error: {}", e)))?;
    match PublicKey::from_bytes(&data) {
        Ok(k) => Ok(k),
        Err(e) => Err(de::Error::custom(format!("{}", e)))
    }
}

fn deserialize_b58_priv<'de, D>(deserializer: D) -> Result<SecretKey, D::Error>
where D: de::Deserializer<'de> {
    let buf = String::deserialize(deserializer)?;
    let mut data = [0u8; ed25519_dalek::SECRET_KEY_LENGTH];
    let _ = bs58::decode(buf).into(&mut data).map_err(|e| de::Error::custom(format!("base58 decode error: {}", e)))?;
    match SecretKey::from_bytes(&data) {
        Ok(k) => Ok(k),
        Err(e) => Err(de::Error::custom(format!("{}", e)))
    }
}

pub struct SMCU {
    keypair: Keypair,
}

#[repr(C)]
pub struct LoraPacket {
    data: *const u8,
    data_len: u32,
    freq: u32, // in 100 KHz

}

fn read_config_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Config> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut config_str = String::new();
    let _ = reader.read_to_string(&mut config_str)?;
    let config: Config = toml::from_str(&config_str)?;
    return Ok(config);
}

#[no_mangle]
pub extern "C"
fn smcu_init(smcu_ptr: &mut *mut SMCU) -> i32 {

    let smcu = match read_config_from_file("smcu.toml") {
        Ok(config) => {
            Box::new(SMCU {
                keypair: Keypair {
                    public: config.pub_key,
                    secret: config.priv_key,
                }
            })
        },

        Err(e) => {
            error!("error reading config file: {}", e);
            let mut csprng = OsRng{};
            Box::new(SMCU {
                keypair: Keypair::generate(&mut csprng),
            })
        }
    };

    *smcu_ptr = Box::into_raw(smcu);
    
    return SMCU_OK;
}

#[no_mangle]
pub extern "C"
fn smcu_free(smcu_ptr: *mut SMCU) {
    let _ = unsafe { Box::from_raw(smcu_ptr) };
}

#[no_mangle]
pub extern "C"
fn smcu_sign(smcu_ptr: *mut SMCU, message_str: *const c_char, pkt_ptr: *const LoraPacket) -> i32 {
    let smcu = unsafe { &mut *smcu_ptr };
    let message = unsafe { CStr::from_ptr(message_str) };
    let pkt = unsafe { & *pkt_ptr };

    

    let signature = smcu.keypair.sign(message.to_bytes());
    
    return SMCU_OK;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
