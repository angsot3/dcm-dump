use std::io::{self, Read};

#[derive(Debug, Clone)]
pub struct DataElement {
    pub tag: (u16, u16),
    pub vr: [u8; 2],
    pub length: u32,
    pub value: Vec<u8>,
}

/// Reads the next data element assuming Explicit VR Little Endian encoding.
pub fn read_element_explicit_le<R: Read>(reader: &mut R) -> io::Result<Option<DataElement>> {
    let mut tag_bytes = [0u8; 4];
    if !fill(reader, &mut tag_bytes)? {
        return Ok(None);
    }

    let group = u16::from_le_bytes([tag_bytes[0], tag_bytes[1]]);
    let element = u16::from_le_bytes([tag_bytes[2], tag_bytes[3]]);

    let mut vr = [0u8; 2];
    fill(reader, &mut vr)?;

    let length = if uses_32bit_length(&vr) {
        let mut reserved = [0u8; 2];
        fill(reader, &mut reserved)?;

        let mut len_bytes = [0u8; 4];
        fill(reader, &mut len_bytes)?;
        u32::from_le_bytes(len_bytes)
    } else {
        let mut len_bytes = [0u8; 2];
        fill(reader, &mut len_bytes)?;
        u16::from_le_bytes(len_bytes) as u32
    };

    let mut value = vec![0u8; length as usize];
    if length > 0 {
        read_exact(reader, &mut value)?;
    }

    Ok(Some(DataElement {
        tag: (group, element),
        vr,
        length,
        value,
    }))
}

pub fn read_transfer_syntax_uid<R: Read>(reader: &mut R) -> io::Result<Option<String>> {
    while let Some(element) = read_element_explicit_le(reader)? {
        if element.tag.0 != 0x0002 {
            // Outside File Meta Information group; stop searching
            return Ok(None);
        }

        if element.tag == (0x0002, 0x0010) {
            let trimmed = element
                .value
                .split(|&b| b == 0)
                .next()
                .unwrap_or(&[])
                .to_vec();
            let as_str = String::from_utf8_lossy(&trimmed).to_string();
            return Ok(Some(as_str));
        }
    }

    Ok(None)
}

fn uses_32bit_length(vr: &[u8; 2]) -> bool {
    matches!(
        std::str::from_utf8(vr).ok(),
        Some("OB" | "OD" | "OF" | "OL" | "OW" | "SQ" | "UC" | "UR" | "UT" | "UN")
    )
}

fn fill<R: Read>(reader: &mut R, buf: &mut [u8]) -> io::Result<bool> {
    let mut read = 0usize;
    while read < buf.len() {
        match reader.read(&mut buf[read..])? {
            0 if read == 0 => return Ok(false),
            0 => {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "unexpected EOF while reading element",
                ))
            }
            n => read += n,
        }
    }
    Ok(true)
}

fn read_exact<R: Read>(reader: &mut R, buf: &mut [u8]) -> io::Result<()> {
    let mut offset = 0usize;
    while offset < buf.len() {
        match reader.read(&mut buf[offset..])? {
            0 => {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "unexpected EOF while reading element value",
                ))
            }
            n => offset += n,
        }
    }
    Ok(())
}
