use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Preamble([u8; 128]);

impl Preamble {
    pub fn new(bytes: [u8; 128]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 128] {
        &self.0
    }

    pub fn is_zeroed(&self) -> bool {
        self.0.iter().all(|&b| b == 0)
    }

    pub fn ascii_preview(&self) -> Option<String> {
        if !self
            .0
            .iter()
            .all(|&b| b == 0 || b.is_ascii_graphic() || b.is_ascii_whitespace())
        {
            return None;
        }

        let text = String::from_utf8_lossy(&self.0);
        let trimmed = text
            .trim_end_matches('\0')
            .trim_end_matches(|c: char| c.is_ascii_control());

        Some(trimmed.to_string())
    }

    pub fn non_zero_len(&self) -> usize {
        self.0.iter().filter(|&&b| b != 0).count()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Prefix([u8; 4]);

impl Prefix {
    pub fn new(bytes: [u8; 4]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 4] {
        &self.0
    }

    pub fn as_str(&self) -> Option<&str> {
        std::str::from_utf8(&self.0).ok()
    }

    pub fn is_dicom(&self) -> bool {
        self.0 == *b"DICM"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PreambleInfo {
    pub preamble: Option<Preamble>,
    pub prefix: Option<Prefix>,
}

pub fn read_preamble_and_prefix<R: Read>(reader: &mut R) -> io::Result<PreambleInfo> {
    let mut info = PreambleInfo::default();

    let mut preamble = [0u8; 128];
    match reader.read_exact(&mut preamble) {
        Ok(()) => info.preamble = Some(Preamble::new(preamble)),
        Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => return Ok(info),
        Err(err) => return Err(err),
    }

    let mut prefix = [0u8; 4];
    match reader.read_exact(&mut prefix) {
        Ok(()) => info.prefix = Some(Prefix::new(prefix)),
        Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => {}
        Err(err) => return Err(err),
    }

    Ok(info)
}
