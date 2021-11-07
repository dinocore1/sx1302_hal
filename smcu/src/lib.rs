extern crate rand;
extern crate ed25519_dalek;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use ed25519_dalek::{Keypair, Signer};
use rand::rngs::OsRng;

pub const SMCU_OK: i32 = 0;


pub struct SMCU {
    keypair: Keypair,
}

#[repr(C)]
pub struct LoraPacket {
    data: *const u8,
    data_len: u32,
    freq: u32, // in 100 KHz

}

#[no_mangle]
pub extern "C"
fn smcu_init(smcu: &mut *mut SMCU) -> i32 {
    let mut csprng = OsRng{};

    *smcu = Box::into_raw(Box::new(SMCU {
        keypair: Keypair::generate(&mut csprng),
    }));
    
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
