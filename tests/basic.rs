use std::fs::File;
use std::io::BufReader;

use dcm_dump::{read_preamble_and_prefix, read_transfer_syntax_uid};

#[test]
fn reads_preamble_prefix_and_transfer_syntax() -> std::io::Result<()> {
    let file = File::open("test.DCM")?;
    let mut reader = BufReader::new(file);

    let info = read_preamble_and_prefix(&mut reader)?;

    let preamble = info
        .preamble
        .expect("preamble bytes should be populated when present");
    assert!(!preamble.is_zeroed());
    assert!(preamble
        .ascii_preview()
        .as_deref()
        .unwrap_or_default()
        .starts_with("Rubo"));

    let prefix = info.prefix.expect("prefix should be present");
    assert!(prefix.is_dicom());

    let uid =
        read_transfer_syntax_uid(&mut reader)?.expect("transfer syntax UID should be present");
    assert_eq!(uid.trim_end_matches('\0'), "1.2.840.10008.1.2.1");

    Ok(())
}
