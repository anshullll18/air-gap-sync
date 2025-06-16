mod encrypt;
mod compress;
mod qr;
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
            let password = "password123"; // You can change this later
            let (encrypted_data, nonce) = encrypt::encrypt(&compressed, password);

            println!("Encrypted size: {} bytes", encrypted_data.len());
            println!("Nonce used: {:?}", nonce);

            // 🧩 Step 9: Combine nonce + encrypted data
            let mut full_payload = nonce.to_vec();
            full_payload.extend_from_slice(&encrypted_data);

            // 📤 Show QR codes
            if via == "qr" {
                qr::display_qr_chunks(&full_payload);
            }
        }

        Commands::Receive { via } => {
            println!("Receiving via: {}", via);

            if via == "qr" {
                // Step 1: Scan chunks
                let full_payload = qr::read_qr_chunks();

                // Step 2: Split nonce + encrypted
                let (nonce, encrypted_data) = full_payload.split_at(12); // 96-bit nonce = 12 bytes

                // Step 3: Decrypt
                let password = "password123"; // Must match sender
                let decrypted = encrypt::decrypt(encrypted_data, nonce.try_into().unwrap(), password);

                // Step 4: Decompress
                let original = compress::decompress_bytes(&decrypted).expect("Decompression failed");

                // Step 5: Save it
                std::fs::write("received.txt", &original).expect("Failed to save output file");

                println!("✅ File received and saved as 'received.txt'");
            }
        }
    }
}