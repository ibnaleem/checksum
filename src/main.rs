// Written by: Ibn Aleem (https://github.com/ibnaleem)
// Issues: https://github.com/ibnaleem/checksum/issues

use md5;
use std::env;
use sha1::Sha1;
use regex::Regex;
use tiger::Tiger;
use std::fs::File;
use colored::Colorize;
use std::io::{BufReader, Read};
use sha2::{Sha224, Sha256, Sha384, Sha512, Digest};
use ripemd::{Ripemd160, Ripemd320};

fn identify_hash(hash: &String) -> String {
    let hash_patterns = [
        (r"^[a-f0-9]{32}$", "MD5"),     
        (r"^[a-f0-9]{40}$", "SHA1"),
        (r"^[a-f0-9]{48}$", "Tiger192"),
        (r"^[a-f0-9]{56}$", "SHA224"),     
        (r"^[a-f0-9]{64}$", "SHA256"),
        (r"^[a-f0-9]{80}$", "RIPEMD320"),
        (r"^[a-f0-9]{96}$", "SHA384"),   
        (r"^[a-f0-9]{128}$", "SHA512"),  
        (r"^[a-f0-9]{128}$", "BLAKE2b"),
    ];

    for (pattern, hash_type) in hash_patterns {
        if Regex::new(pattern).unwrap().is_match(&hash) {
            return hash_type.to_string();
        }
    }

    return "Unknown".to_string()
}


fn calculate_hash(hash_type: &String, filepath: &String) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1048576];

    let mut hasher: Box<dyn digest::DynDigest> = match hash_type.to_lowercase().as_str() {
        "md5" => Box::new(md5::Md5::new()),
        "sha1" => Box::new(Sha1::new()),
        "sha224" => Box::new(Sha224::new()),
        "sha256" => Box::new(Sha256::new()),
        "sha384" => Box::new(Sha384::new()),
        "sha512" => Box::new(Sha512::new()),
        "tiger192" => Box::new(Tiger::new()),
        "ripemd160" => Box::new(Ripemd160::new()),
        "ripemd320" => Box::new(Ripemd320::new()),
        _ => return Err("Unsupported hash type".into()),
    };

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(hex::encode(result))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("{} {} {}", "Usage:".red(), args[0].red(), "<filepath> <hash>".red());
        return;
    }

    let filepath = args[1].clone();
    let hash = args[2].clone();
    let hash_type = identify_hash(&hash);
    
    println!("{} {} {} {} {}", 
        "Using".yellow(), 
        &hash_type.yellow(), 
        "algorithm to calculate hash for".yellow(), 
        &filepath.yellow(), 
        "...".yellow()
    );

    let hash_result = calculate_hash(&hash_type, &filepath);

    match hash_result {
        Ok(result) => {
            if result == hash {
                println!("{} {} {} {} {}", 
                    "Checksum matches for".green(), 
                    filepath.green(), 
                    "using".green(), 
                    hash_type.green(), 
                    "algorithm!".green()
                );
                println!("{} {}", "Provided hash:".green(), hash.green());
                println!("{} {}", "Calculated hash:".green(), result.green());
            } else {
                println!("{} {} {} {} {}", 
                    "Checksum does not match for".red(), 
                    filepath.red(), 
                    "using".red(), 
                    hash_type.red(), 
                    "algorithm!".red()
                );
                println!("{} {}", "Provided hash:".red(), hash.red());
                println!("{} {}", "Calculated hash:".red(), result.red());
            }
        },
        Err(e) => println!("{} {}", "Error calculating hash:".red(), e),
    }
}
