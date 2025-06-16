use std::fs::File;
use std::io::{self, Read, Write, Cursor};
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;

/// Compresses raw bytes using GZIP
pub fn compress_bytes(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    encoder.finish()
}

/// Decompresses GZIP-compressed bytes
pub fn decompress_bytes(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut decoder = GzDecoder::new(Cursor::new(data));
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}