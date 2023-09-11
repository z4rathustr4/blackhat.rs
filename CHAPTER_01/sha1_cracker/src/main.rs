use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const SHA1_HEX_STRING_LEN: usize = 40;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <SHA-1_HASH>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LEN {
        return Err("SHA-1 Hash is not valid".into());
    }

    let wordlist_file = File::open(&args[1])?;
    let reader        = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        let line = line?;
        let common_passwd = line.trim();
        
        if hash_to_crack == hex::encode(sha1::Sha1::digest(common_passwd.as_bytes())) {
            println!("[+] Password found: {}", &common_passwd);
            return Ok(())
        } else {
            println!("[E]: Password not found with provided wordlist.");
        }
    }

    Ok(())
}
