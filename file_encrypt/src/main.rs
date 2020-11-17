use ring::{pbkdf2,digest,rand::{self,SecureRandom}};
use std::{num::NonZeroU32, collections::HashMap,fs};
use std::{io::{self,prelude::*}};
mod crypto;
use crypto::*;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA512;
const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
type Credential = [u8;CREDENTIAL_LEN];
struct PasswordDB {
    pbkdf2_interations: NonZeroU32,
}
impl PasswordDB {
    pub fn store_password(&mut self, password:&str,salt:&Vec<u8>){
        let mut to_store: Credential = [0u8;CREDENTIAL_LEN];
        pbkdf2::derive(
            PBKDF2_ALG,
            self.pbkdf2_interations,
            &salt, 
            password.as_bytes(), 
            &mut to_store);
    }
    fn verify_password(&mut self, password:&str,salt:&Vec<u8>){
        let mut to_verify: Credential = [0u8;CREDENTIAL_LEN];
            pbkdf2::verify(
                PBKDF2_ALG,
                self.pbkdf2_interations,
                &salt, 
                password.as_bytes(),
                &mut to_verify);
    }
}

fn salt() -> Vec<u8> {
    let rng = rand::SystemRandom::new();
    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt)
        .expect("Err to generate salt");
    salt.to_vec()
}
fn store(){
    let mut db = PasswordDB {
        pbkdf2_interations: NonZeroU32::new(100_000).unwrap(),
    };
    let mut password =String::new();
    io::stdin().read_line(&mut password)
        .expect("Err to read_line");
    let salt = salt();
    db.store_password(&password,&salt);
    let mut file = fs::File::create("salt")
        .expect("can not create file");
   
    for i in 0..salt.len(){
        let buf = salt.get(i).unwrap();
        write!(file,"{}",buf);
    }
}
fn verify() {
    let salt = open_file();
    let mut db = PasswordDB {
        pbkdf2_interations: NonZeroU32::new(100_000).unwrap(),
    };
    let mut password =String::new();
    io::stdin().read_line(&mut password)
        .expect("Err to read_line");
    db.verify_password(&password,&salt);

}
fn open_file() -> Vec<u8>{
    let file = fs::open();
    let salt = fs::read(file).unwrap();
salt
}

fn main(){
    store();
}
