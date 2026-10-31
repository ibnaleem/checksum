// Written by: Ibn Aleem (https://github.com/ibnaleem)
// Issues: https://github.com/ibnaleem/checksum/issues

use md5;
use std::env;
use sha1::Sha1;
use tiger::Tiger;
use std::fs::File;
use colored::Colorize;
use std::io::Read;
use sha2::{Sha224, Sha256, Sha384, Sha512, Digest};
use ripemd::{Ripemd160, Ripemd320};

const BUFFER_SIZE: usize = 1 << 20;

fn identify_hash(hash: &str) -> String {
    if !hash.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
        return "Unknown".to_string();
    }
    match hash.len() {
        32 => "MD5",
        40 => "SHA1",
        48 => "Tiger192",
        56 => "SHA224",
        64 => "SHA256",
        80 => "RIPEMD320",
        96 => "SHA384",
        128 => "SHA512",
        _ => "Unknown",
    }
    .to_string()
}

fn calculate_hash(hash_type: &str, filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(filepath)?;
    // Heap, not stack: a 1 MiB array overflows the 1 MiB Windows main-thread stack.
    let mut buffer = vec![0u8; BUFFER_SIZE];

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
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hex::encode(hasher.finalize()))
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

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_file(name: &str, data: &[u8]) -> String {
        let path = env::temp_dir().join(name);
        std::fs::write(&path, data).unwrap();
        path.to_string_lossy().into_owned()
    }

    #[test]
    fn identifies_hash_by_length() {
        assert_eq!(identify_hash(&"a".repeat(32)), "MD5");
        assert_eq!(identify_hash(&"0".repeat(64)), "SHA256");
        assert_eq!(identify_hash(&"f".repeat(128)), "SHA512");
        assert_eq!(identify_hash(&"A".repeat(32)), "Unknown");
        assert_eq!(identify_hash(&"g".repeat(32)), "Unknown");
        assert_eq!(identify_hash("abc"), "Unknown");
    }

    #[test]
    fn hashes_known_vectors() {
        let path = temp_file("checksum_test_abc", b"abc");
        assert_eq!(calculate_hash("MD5", &path).unwrap(), "900150983cd24fb0d6963f7d28e17f72");
        assert_eq!(calculate_hash("SHA1", &path).unwrap(), "a9993e364706816aba3e25717850c26c9cd0d89d");
        assert_eq!(
            calculate_hash("SHA256", &path).unwrap(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert!(calculate_hash("Unknown", &path).is_err());
    }

    #[test]
    fn hashes_files_larger_than_buffer() {
        let data: Vec<u8> = (0..BUFFER_SIZE * 3 + 7).map(|i| i as u8).collect();
        let path = temp_file("checksum_test_big", &data);
        assert_eq!(calculate_hash("SHA256", &path).unwrap(), hex::encode(Sha256::digest(&data)));
    }
}
