use std::env;
use std::fs::File;
use std::io::{self, BufReader};

use dcm_dump::{read_preamble_and_prefix, read_transfer_syntax_uid, Preamble, Prefix};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file.dcm>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let info = read_preamble_and_prefix(&mut reader)?;
    println!("File: {}", filename);

    if let Some(preamble) = info.preamble {
        report_preamble(&preamble);
    } else {
        println!("Preamble: missing or invalid");
    }

    if let Some(prefix) = info.prefix {
        report_prefix(&prefix);
    } else {
        println!("Prefix:   <missing>");
    }

    match read_transfer_syntax_uid(&mut reader)? {
        Some(uid) => println!("Transfer Syntax UID: {}", uid),
        None => println!("Transfer Syntax UID: <not found>"),
    }

    Ok(())
}

fn report_preamble(preamble: &Preamble) {
    if preamble.is_zeroed() {
        println!("Preamble: present (128 bytes, empty as expected)");
        return;
    }

    match preamble.ascii_preview() {
        Some(preview) if preview.is_empty() => {
            println!("Preamble: present (128 bytes, empty as expected)");
        }
        Some(preview) => println!("Preamble: present (128 bytes) -> \"{}\"", preview),
        None => {
            let non_zero = preamble.non_zero_len();
            println!("Preamble: present (128 bytes, {non_zero} non-zero bytes)");
        }
    }
}

fn report_prefix(prefix: &Prefix) {
    match prefix.as_str() {
        Some(text) if prefix.is_dicom() => println!("Prefix:   \"{}\" ✅", text),
        Some(text) => println!("Prefix:   \"{}\" ❌ (expected \"DICM\")", text),
        None => println!("Prefix:   <non-UTF8 bytes {:02X?}>", prefix.as_bytes()),
    }
}
