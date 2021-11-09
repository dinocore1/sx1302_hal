use std::io::BufWriter;
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

use serde::{de, Serialize, Deserialize, ser::Serializer};

use log::{info, warn, error};



pub const SMCU_OK: i32 = 0;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    #[serde(serialize_with = "serialize_b58")]
    #[serde(deserialize_with = "deserialize_b58_priv")]
    secret_key: SecretKey,
}

fn serialize_b58<S, T>(input: &T, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer,
T: AsRef<[u8]> {
    serializer.serialize_str(&bs58::encode(input).into_string())
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

fn write_config_file<P: AsRef<Path>>(path: P, config: &Config) -> std::io::Result<()> {
    let config_str = toml::to_string_pretty(config)
                                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(config_str.as_bytes())
}

#[no_mangle]
pub extern "C"
fn smcu_init(smcu_ptr: &mut *mut SMCU) -> i32 {

    const CONFIG_FILE_PATH: &str = "smcu.toml";
    let smcu = match read_config_from_file(CONFIG_FILE_PATH) {
        Ok(config) => {
            Box::new(SMCU {
                keypair: Keypair {
                    public: PublicKey::from(&config.secret_key),
                    secret: config.secret_key,
                }
            })
        },

        Err(e) => {
            error!("error reading config file: {}", e);
            let mut csprng = OsRng{};
            let keypair = Keypair::generate(&mut csprng);

            let config = Config {
                secret_key: keypair.secret,
            };
            

            if let Err(e) = write_config_file(CONFIG_FILE_PATH, &config) {
                error!("error writing config file: {}", e);
            }

            Box::new(SMCU {
                keypair: Keypair {
                    public: keypair.public,
                    secret: config.secret_key,
                },
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
    use simple_logger::SimpleLogger;

    #[test]
    fn it_works() {
        SimpleLogger::new().init().unwrap();
        let mut smcu = 0 as *mut SMCU;
        smcu_init(&mut smcu);

        smcu_free(smcu);
        
    }

    #[test]
    fn derived_is_same() {
        let mut csprng = OsRng{};
        let keypair = Keypair::generate(&mut csprng);

        let derive_pub = PublicKey::from(&keypair.secret);

        assert_eq!(derive_pub, keypair.public);

    }
}
