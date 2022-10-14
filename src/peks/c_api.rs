use paired::bls12_381::Bls12;
use rand_core::OsRng;
use std::ffi::*;
use std::marker::PhantomData;
use std::mem;
use std::os::raw::c_char;
use std::slice;
use std::str::FromStr;

use crate::c_utils::*;
use crate::peks::*;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CPeksSecretKey {
    ptr: *mut c_char,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CPeksPublicKey {
    ptr: *mut c_char,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CPeksCiphertext {
    ptr: *mut c_char,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CPeksTrapdoor {
    ptr: *mut c_char,
}

#[no_mangle]
pub extern "C" fn gen_secret_key() -> CPeksSecretKey {
    let mut rng = OsRng;
    let sk = SecretKey::<Bls12>::gen(&mut rng);
    let sk_str = serde_json::to_string(&sk).unwrap();
    CPeksSecretKey {
        ptr: str2ptr(sk_str),
    }
}

#[no_mangle]
pub extern "C" fn peks_gen_public_key(secret_key: &CPeksSecretKey) -> CPeksPublicKey {
    let mut rng = OsRng;
    let sk = serde_json::from_str::<SecretKey<Bls12>>(&ptr2str(secret_key.ptr)).unwrap();
    let pk = sk.into_public_key(&mut rng);
    let pk_str = serde_json::to_string(&pk).unwrap();
    CPeksPublicKey {
        ptr: str2ptr(pk_str),
    }
}

#[no_mangle]
pub extern "C" fn peks_encrypt_keyword(
    public_key: &CPeksPublicKey,
    keyword: *mut c_char,
) -> CPeksCiphertext {
    let mut rng = OsRng;
    let pk = serde_json::from_str::<PublicKey<Bls12>>(&ptr2str(public_key.ptr)).unwrap();
    let keyword = ptr2str(keyword).as_bytes();
    let ct = pk.encrypt(keyword, &mut rng).unwrap();
    let ct_str = serde_json::to_string(&ct).unwrap();
    CPeksCiphertext {
        ptr: str2ptr(ct_str),
    }
}

#[no_mangle]
pub extern "C" fn peks_gen_trapdoor(
    secret_key: &CPeksSecretKey,
    keyword: *mut c_char,
) -> CPeksTrapdoor {
    let sk = serde_json::from_str::<SecretKey<Bls12>>(&ptr2str(secret_key.ptr)).unwrap();
    let keyword = ptr2str(keyword).as_bytes();
    let td = sk.gen_trapdoor(keyword);
    let td_str = serde_json::to_string(&td).unwrap();
    CPeksTrapdoor {
        ptr: str2ptr(td_str),
    }
}

#[no_mangle]
pub extern "C" fn peks_test(ciphertext: CPeksCiphertext, trapdoor: CPeksTrapdoor) -> bool {
    let ct = serde_json::from_str::<Ciphertext<Bls12>>(&ptr2str(ciphertext.ptr)).unwrap();
    let td = serde_json::from_str::<Trapdoor<Bls12>>(&ptr2str(trapdoor.ptr)).unwrap();
    let tested = td.test(&ct).unwrap();
    tested
}

#[no_mangle]
pub extern "C" fn peks_free_secret_key(secret_key: CPeksSecretKey) {
    drop_ptr(secret_key.ptr);
}

#[no_mangle]
pub extern "C" fn peks_free_public_key(public_key: CPeksPublicKey) {
    drop_ptr(public_key.ptr);
}

#[no_mangle]
pub extern "C" fn peks_free_ciphertext(ciphertext: CPeksCiphertext) {
    drop_ptr(ciphertext.ptr);
}

#[no_mangle]
pub extern "C" fn peks_free_trapdoor(trapdoor: CPeksTrapdoor) {
    drop_ptr(trapdoor.ptr);
}
