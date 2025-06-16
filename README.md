# AirGapSync

A secure file transfer tool for air-gapped systems using QR codes with compression and encryption.

## Features

- **Secure**: AES-256-GCM encryption with password protection
- **Compressed**: GZIP compression to minimize QR code size
- **Air-gapped**: Transfer files via QR codes displayed in terminal
- **Flexible scanning**: Manual paste or image file QR decoding

## Installation

```bash
git clone <repo>
cd air-gap-sync
cargo build --release
```

## Usage

### Send a file

```bash
cargo run -- send <file> --via qr
```

Enter encryption password when prompted. QR codes will be displayed in chunks.

### Receive a file

```bash
# Manual paste mode (copy/paste base64)
cargo run -- receive --via qr

# Enhanced mode (scan from image files)
cargo run -- receive --via qr --enhanced
```

Enter the same password used for encryption. Output saved to `received_output.txt`.

## How it works

1. **Send**: File → Compress → Encrypt → Split into QR chunks → Display
2. **Receive**: Scan QR chunks → Combine → Decrypt → Decompress → Save file

## Security

- Uses AES-256-GCM authenticated encryption
- Password-derived keys via SHA-256
- Random nonces for each encryption

## Dependencies

- Rust 2021 edition
- Key crates: `aes-gcm`, `flate2`, `qrcode`, `rqrr`, `image`
