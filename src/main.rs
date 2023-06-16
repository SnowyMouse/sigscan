extern crate clap;
use clap::Parser;

use std::fs::File;
use std::io::{BufReader, Read};
use std::process::exit;

#[derive(Parser)]
#[clap(version, about)]
struct Args {
    /// Path to the file to scan
    file_path: String,

    /// Signature to look for (e.g. "00??AABBCCDD")
    signature: String,

    /// Number of matches (printing on each line). Set to 0 to find all.
    #[clap(short, long, default_value_t = 1)]
    matches: u64,

    /// Print the result(s) in decimal instead of hexadecimal.
    #[clap(short, long)]
    decimal: bool
}

fn main() -> std::io::Result<()>  {
    let args = Args::parse();

    // Turn the string into a signature
    fn parse_string_into_signature(signature: &str) -> Option<Vec<Option<u8>>> {
        let chars : Vec<char> = signature.replace(" ", "").chars().collect(); // remove whitespace. Convert to array
        let char_count = chars.len();

        // Is it divisible by 2?
        if char_count % 2 != 0 {
            return None;
        }

        // Is it greater than 0?
        if char_count == 0 {
            return None;
        }

        // Let's start
        let mut signature = Vec::new();
        signature.reserve(char_count / 2);

        for i in (0..chars.len()).step_by(2) {
            let high = chars[i];
            let low = chars[i+1];

            if high == '?' && low == '?' {
                signature.push(None)
            }
            else {
                signature.push(Some(((high.to_digit(16)? << 4) | (low.to_digit(16)?)) as u8));
            }
        }

        Some(signature)
    }

    // Read the signature
    let signature = match parse_string_into_signature(&args.signature) {
        Some(n) => n,
        None => {
            eprintln!("Failed to parse signature");
            return Ok(())
        }
    };

    // Load the file
    let mut file = BufReader::new(File::open(&args.file_path)?);
    let sig_length = signature.len();

    // Loop until we are done
    let mut testing = Vec::<u64>::new();
    let mut confirmed = Vec::<u64>::new();
    let mut current_offset = 0u64;

    while args.matches == 0 || confirmed.len() < args.matches as usize {
        let mut buffer = [0u8; 1];
        if file.read(&mut buffer)? == 0 {
            break;
        }
        let buffer = buffer[0];

        let matches = |sig_offset: u64| -> bool {
            match signature[sig_offset as usize] {
                Some(n) => n == buffer,
                None => true
            }
        };

        testing.retain(|u| {
            let sig_offset = current_offset - u;
            if !matches(sig_offset) {
                false
            }
            else if sig_offset + 1 == sig_length as u64 {
                confirmed.push(*u);
                false
            }
            else {
                true
            }
        });

        if matches(0) {
            if sig_length == 1 {
                confirmed.push(current_offset);
            }
            else {
                testing.push(current_offset);
            }
        }

        current_offset += 1;
    }

    // Check if we matched anything
    if confirmed.is_empty() {
        println!("none");
        exit(1);
    }

    // Print offsets
    if args.decimal {
        for i in confirmed {
            println!("{i}");
        }
    }
    else {
        for i in confirmed {
            println!("{i:X}");
        }
    }

    Ok(())
}
