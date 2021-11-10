mod loradatarate;

use loradatarate::*;
use rand::RngCore;


use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::fs::File;
use std::ffi::{CStr};
use std::os::raw::c_char;

use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signer};
use rand::rngs::OsRng;
use toml::{self};
use serde::{de, Serialize, Deserialize, ser::Serializer};
use log::{info, warn, error};
use bytes::{BufMut, BytesMut};

pub const SMCU_OK: i32 = 0;
pub const SMCU_ERR: i32 = -1;
pub const SIGNATURE_LENGTH: usize = 64;
pub type signature_t = [u8;SIGNATURE_LENGTH];

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    #[serde(serialize_with = "serialize_b58")]
    #[serde(deserialize_with = "deserialize_b58_priv")]
    secret_key: SecretKey,

    hardware_id: String,
}

fn serialize_b58<S, T>(input: &T, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer,
T: AsRef<[u8]> {
    serializer.serialize_str(&bs58::encode(input).into_string())
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
    hardware_id: String,
}

#[repr(C)]
pub struct LoraPacket {
    data: [u8 ; 256],
    data_len: u16,
    tmstmp: u32,
    rssis: f32,
    snr: f32,
    freq_hz: u32,
    bandwidth: u8,
    datarate: u8,
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
                },
                hardware_id: config.hardware_id,
            })
        },

        Err(e) => {
            error!("error reading config file: {}", e);
            let mut csprng = OsRng{};
            let keypair = Keypair::generate(&mut csprng);

            let config = Config {
                secret_key: keypair.secret,
                hardware_id: format!("A{:0>8}", csprng.next_u32())
            };
            

            if let Err(e) = write_config_file(CONFIG_FILE_PATH, &config) {
                error!("error writing config file: {}", e);
            }

            Box::new(SMCU {
                keypair: Keypair {
                    public: keypair.public,
                    secret: config.secret_key,
                },
                hardware_id: config.hardware_id,
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
fn smcu_sign(smcu_ptr: *mut SMCU, sig: &mut signature_t, pkt_ptr: *const LoraPacket) -> i32 {
    let smcu = unsafe { &mut *smcu_ptr };
    //let message = unsafe { CStr::from_ptr(message_str) };
    let pkt = unsafe { & *pkt_ptr };

    let mut message = BytesMut::new();

    message.put(&pkt.data[..pkt.data_len as usize]);
    message.put_u32(pkt.data_len as u32);
    message.put_u32(pkt.tmstmp);


    let rssi = round_float(pkt.rssis, 0);
    message.put_i32(rssi);

    let snr = round_float(pkt.snr, 1);
    message.put_i32(snr);

    let dr = match LoraDatarate::try_from(pkt.datarate) {
        Ok(dr) => dr,
        Err(e) => {
            error!("error decoding datarate: {}", e);
            return SMCU_ERR;
        }
    };

    let bw = match LoraBandwidth::try_from(pkt.bandwidth) {
        Ok(bw) => bw,
        Err(e) => {
            error!("error decoding bandwith: {}", e);
            return SMCU_ERR;
        }
    };

    let datr = Datr::new(dr, bw);
    let datr_str = datr.to_string();
    message.put(datr_str.as_bytes());
    

    let freq_khz = pkt.freq_hz / 1000;
    message.put_u32(freq_khz);

    message.put(smcu.hardware_id.as_bytes());

    let signature = smcu.keypair.sign(&message[..]);
    sig.clone_from_slice(signature.as_ref());
    return SMCU_OK;
}

fn round_float(v: f32, i: i32) -> i32 {
    (v * 10.0_f32.powi(i)).round() as i32
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

    #[test]
    fn round_float_test() {
        assert_eq!(90252, round_float(902.5246777_f32, 2));
        assert_eq!(-421, round_float(-42.12222_f32, 1));
    }

    #[test]
    fn test_freq_float() {
        let f_mhz = 902.5246777_f32;
        let f_khz = f_mhz * 1000_f32;
        let r = f_khz.round() as u32;
        assert_eq!(902525_u32, r);
    }

    #[test]
    fn test_freq_int() {
        let f_mhz = 902_525_000_u32;
        let f_khz = f_mhz / 1000;
        assert_eq!(902525_u32, f_khz);
    }

    #[test]
    fn put_i32_eq_u32() {
        let mut a = BytesMut::new();
        a.put_i32(-13_i32);

        let mut b = BytesMut::new();
        b.put_u32(-13_i32 as u32);

        assert_eq!(&a, &b);
    }
}
