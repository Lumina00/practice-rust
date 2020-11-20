use ring::{pbkdf2,digest,rand::{self,SecureRandom}};
use std::num::NonZeroU32;
use std::{fs,str};
use std::{io::{stdin,prelude::*},path::PathBuf};

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA512;
const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
type Credential = [u8;CREDENTIAL_LEN];
pub struct PasswordDB {
    pbkdf2_interations: NonZeroU32,
}
impl PasswordDB {
    pub fn store_password(&mut self, password:&str,salt:&Vec<u8>)->Credential{
        let mut to_store: Credential = [0u8;CREDENTIAL_LEN];
        pbkdf2::derive(
            PBKDF2_ALG,
            self.pbkdf2_interations,
            &salt, 
            password.as_bytes(), 
            &mut to_store);
        to_store
    }
    pub fn verify_password(&mut self, password:&str,salt:&Vec<u8>,to_verify:&mut Vec<u8>){
        let qwe = pbkdf2::verify(
            PBKDF2_ALG,
            self.pbkdf2_interations,
            &salt, 
            password.as_bytes(),
            to_verify);
    }
}
fn salt() -> Vec<u8> {
    let rng = rand::SystemRandom::new();
    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt)
        .expect("Err to generate salt");
    salt.to_vec()
}
pub fn store(){
    let mut db = PasswordDB {
        pbkdf2_interations: NonZeroU32::new(100_000).unwrap(),
    };
    let mut password =String::new();
    stdin().read_line(&mut password)
        .expect("Err to read_line");
    let salt = salt();
    let store = db.store_password(&password,&salt);
    let mut file_salt = fs::File::create("salt")
        .expect("can not create file");
    let mut file_store = fs::File::create("store")
        .expect("can not create file");
    let salt:&[u8] = &salt;
    let store:&[u8] = &store;
        unsafe {
            let buf = str::from_utf8_unchecked(salt);
            write!(file_salt,"{}",buf)
                .expect("Err to Write");
            let buf = str::from_utf8_unchecked(store);
            write!(file_store,"{}",buf)
                .expect("Err to Write");
        }
}
pub fn verify(salt:&str,store:&str) {
    let salt = open_file(salt);
    let mut store = open_file(store);
    let mut db = PasswordDB {
        pbkdf2_interations: NonZeroU32::new(100_000).unwrap(),
    };
    let mut password = String::new();
    stdin().read_line(&mut password)
        .expect("Err to read_line");
    let qwe = db.verify_password(&password,&salt,&mut store);
}
pub fn open_file(file:&str) -> Vec<u8>{
    let file = PathBuf::from(file);
    let mut file = fs::File::open(file)
        .expect("Err to open file");
    let mut salt = Vec::new();
        file.read_to_end(&mut salt).unwrap();
salt
}
