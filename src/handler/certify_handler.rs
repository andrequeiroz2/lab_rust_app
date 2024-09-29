use std::{fs::File, io::Read, path::Path};
use super::error_handler::ErrorEnum;
use log::error;
use std::fs::OpenOptions;
use std::env;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref PrivateKey: Vec<u8> = get_private_key();
    pub static ref PublicKey: Vec<u8> = get_public_key();
}


pub fn get_private_key() -> Vec<u8> {

    let file_path = Path::new("./security/private_key.pem");

    let mut options = OpenOptions::new();
    
    let mut file_read = match options.read(true).open(file_path){
        
        Ok(f) => f,

        Err(e) => panic!("{}", e.to_string())
    };
    
    let mut buffer = Vec::new();
    
    let private_key = match file_read.read_to_end(&mut buffer){
        Ok(r) => r.to_le_bytes().to_vec(),
        Err(err)=> panic!("{}", err.to_string())
    };

    private_key
}

pub fn get_public_key() -> Vec<u8> {

    let file_path = Path::new("./security/public_key.pem");

    let mut options = OpenOptions::new();
    
    let mut file_read = match options.read(true).open(file_path){
        
        Ok(f) => f,

        Err(e) => panic!("{}", e.to_string())
    };
    
    let mut buffer = Vec::new();
    
    let public_key = match file_read.read_to_end(&mut buffer){
        Ok(r) => r.to_le_bytes().to_vec(),
        Err(err)=> panic!("{}", err.to_string())
    };

    public_key
}
