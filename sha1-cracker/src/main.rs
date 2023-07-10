use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

// all SHA-1 hashes are 40 chars long
const SHA1_HEX_STR_LEN: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("sha1-cracker <wordlist.txt> <SHA-1 hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();

    if hash_to_crack.len() != SHA1_HEX_STR_LEN {
        return Err("[E]: This is not a valid SHA-1 hash.".into());
    }

    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        let line = line?.trim().to_string();
        println!("{line}");
    }

    Ok(())
}
