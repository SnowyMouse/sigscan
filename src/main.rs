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
    let mut signature_byte_to_check = 0;

    // Loop until we are done
    let mut buffer = [0; 1];
    let mut current_offset = 0u64;
    let mut total_matches = 0u64;

    loop {
        if file.read(&mut buffer)? == 0 {
            break;
        }

        let matches = match signature[signature_byte_to_check] {
            Some(n) => n == buffer[0],
            None => true
        };

        current_offset += 1;

        if matches {
            signature_byte_to_check += 1;
            if signature_byte_to_check == sig_length {
                if args.decimal {
                    println!("{}", current_offset - signature_byte_to_check as u64);
                }
                else {
                    println!("{:016X}", current_offset - signature_byte_to_check as u64);
                }
                total_matches += 1;

                if total_matches == args.matches && args.matches != 0 {
                    break;
                }

                signature_byte_to_check = 0;
            }
        }
        else {
            signature_byte_to_check = 0;
        }
    }

    // Check if we matched anything
    if total_matches != 0 {
        exit(0);
    }
    else {
        println!("none");
        exit(1);
    }
}
