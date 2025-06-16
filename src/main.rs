mod encrypt;
mod compress;
mod qr;
mod scanner;  // New scanner module

use std::fs;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "AirGapSync")]
#[command(about = "Sync files across air-gapped devices", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send a file
    Send {
        #[arg(help = "Path to the file to send")]
        file: String,
        #[arg(long, help = "Transfer method: qr or usb")]
        via: String,
    },

    /// Receive a file
    Receive {
        #[arg(long, help = "Transfer method: qr or usb")]
        via: String,
        #[arg(long, help = "Use enhanced scanner (camera/file/manual)")]
        enhanced: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Send { file, via } => {
            println!("Sending file: {}", file);
            println!("Method: {}", via);

            // 🔽 Step 1: Read file content
            let input_data = fs::read(file).expect("Failed to read file");

            // 📦 Step 2: Compress it
            let compressed = compress::compress_bytes(&input_data).expect("Compression failed");

            println!("Original size: {} bytes", input_data.len());
            println!("Compressed size: {} bytes", compressed.len());

            // 🔐 Step 3: Encrypt it
            use rpassword::prompt_password;
            let password = prompt_password("Enter password for encryption: ").expect("❌ Failed to read password");
            let (encrypted_data, nonce) = encrypt::encrypt(&compressed, &password);

            println!("Encrypted size: {} bytes", encrypted_data.len());
            println!("Nonce used: {:?}", nonce);

            // 🧩 Step 4: Combine nonce + encrypted data
            let mut full_payload = nonce.to_vec();
            full_payload.extend_from_slice(&encrypted_data);

            // 📤 Show QR codes
            if via == "qr" {
                qr::display_qr_chunks(&full_payload);
            }
        }

        Commands::Receive { via, enhanced } => {
            println!("Receiving via: {}", via);
            if via == "qr" {
                // Choose between enhanced scanner or original method
                let encrypted = if *enhanced {
                    println!("🔍 Using enhanced scanner mode");
                    scanner::scan_qr_chunks_enhanced()
                } else {
                    println!("📝 Using manual paste mode");
                    qr::read_qr_chunks()
                };

                if encrypted.len() < 12 {
                    println!("❌ Received data too short (need at least 12 bytes for nonce)");
                    return;
                }

                // ✅ Extract nonce from first 12 bytes
                let nonce_bytes: Vec<u8> = encrypted[..12].to_vec();
                let ciphertext = &encrypted[12..];

                // 3️⃣ Ask for password
                println!("🔑 Enter password used for encryption:");
                let mut password = String::new();
                std::io::stdin().read_line(&mut password).unwrap();
                let password = password.trim();

                // 4️⃣ Decrypt
                println!("Attempting decryption with nonce: {:?}", nonce_bytes);
                println!("Encrypted data length: {}", ciphertext.len());

                let decrypted = encrypt::decrypt_bytes(ciphertext, password, &nonce_bytes)
                    .expect("❌ Decryption failed!");
                println!("✅ Decrypted size: {} bytes", decrypted.len());

                // 5️⃣ Decompress
                let decompressed = compress::decompress_bytes(&decrypted)
                    .expect("❌ Decompression failed!");

                println!("✅ Decompressed size: {} bytes", decompressed.len());

                // 6️⃣ Write to output file
                let output_path = "received_output.txt";
                fs::write(output_path, decompressed).expect("❌ Failed to write file");

                println!("📁 File written to: {}", output_path);
            }
        }
    }
}