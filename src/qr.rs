use qrcode::{QrCode, EcLevel};
use qrcode::render::unicode;
use std::str;

pub fn display_qr_chunks(data: &[u8]) {
    let chunk_size = 1000;
    let total_chunks = (data.len() + chunk_size - 1) / chunk_size;

    for i in 0..total_chunks {
        let start = i * chunk_size;
        let end = usize::min(start + chunk_size, data.len());

        let chunk = &data[start..end];
        let label = format!("QR {}/{}", i + 1, total_chunks);
        println!("\n{}", label);

        // Encode chunk as Base64
        let b64_chunk = base64::encode(chunk);
        println!("\n📋 Pasteable base64 (for QR {}):\n{}", i + 1, b64_chunk);
        // Generate QR code
        let code = QrCode::with_error_correction_level(b64_chunk.as_bytes(), EcLevel::M).unwrap();
        let image = code.render::<unicode::Dense1x2>().build();
        println!("{}", image);

        // Wait for user to proceed
        println!("Press Enter for next QR...");
        let mut dummy = String::new();
        std::io::stdin().read_line(&mut dummy).unwrap();
    }
}
use std::io::{self, Write};

pub fn read_qr_chunks() -> Vec<u8> {
    let mut result = Vec::new();
    let mut chunk_num = 1;

    println!("📥 Paste each QR chunk (base64) and press Enter.");
    println!("Type 'done' when finished.\n");

    loop {
        print!("Chunk {} > ", chunk_num);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("done") {
            break;
        }

        match base64::decode(input) {
            Ok(decoded) => {
                result.extend_from_slice(&decoded);
                println!("✅ Received {} bytes.", decoded.len());
                chunk_num += 1;
            }
            Err(_) => {
                println!("❌ Invalid base64. Try again.");
            }
        }
    }

    println!("📦 Total received bytes: {}", result.len());
    result
}