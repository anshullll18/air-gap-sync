use image::DynamicImage;
use std::io::{self, Write};
use std::path::Path;

pub enum ScanMethod {
    File(String),
    Manual,
}

pub fn get_scan_method() -> ScanMethod {
    println!("\n📱 Choose QR input method:");
    println!("1. Load from image file");
    println!("2. Manual paste (base64)");
    
    loop {
        print!("Enter choice (1-2): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => {
                print!("Enter image file path: ");
                io::stdout().flush().unwrap();
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();
                return ScanMethod::File(path.trim().to_string());
            }
            "2" => return ScanMethod::Manual,
            _ => println!("Invalid choice. Please enter 1 or 2."),
        }
    }
}

pub fn scan_from_file(image_path: &str) -> Result<Option<String>, String> {
    // Check if file exists first
    if !Path::new(image_path).exists() {
        return Err(format!("File does not exist: {}", image_path));
    }
    
    println!("📁 File exists, attempting to load...");
    let img = image::open(image_path)
        .map_err(|e| format!("Failed to open image: {}", e))?;
    
    println!("📐 Image loaded: {}x{}", img.width(), img.height());
    
    // Try multiple preprocessing approaches
    let approaches = vec![
        ("Original grayscale", img.to_luma8()),
        ("High contrast", enhance_contrast(&img)),
        ("Inverted", invert_image(&img)),
    ];
    
    for (name, gray_img) in approaches {
        println!("🔍 Trying {} approach...", name);
        let (width, height) = gray_img.dimensions();
        
        // Use rqrr for file-based QR detection
        let mut prepared_img = rqrr::PreparedImage::prepare_from_greyscale(
            width as usize,
            height as usize,
            |x, y| {
                *gray_img.get_pixel(x as u32, y as u32).0.first().unwrap_or(&0)
            }
        );
        
        let grids = prepared_img.detect_grids();
        println!("🎯 Found {} potential QR grid(s) with {}", grids.len(), name);
        
        if !grids.is_empty() {
            for (i, grid) in grids.iter().enumerate() {
                println!("📋 Attempting to decode grid {} with {}", i + 1, name);
                match grid.decode() {
                    Ok((_, content)) => {
                        println!("✅ Successfully decoded QR code with {}!", name);
                        return Ok(Some(content));
                    }
                    Err(e) => {
                        println!("❌ Failed to decode grid {} with {}: {:?}", i + 1, name, e);
                    }
                }
            }
        }
    }
    
    println!("❌ No QR code could be detected with any preprocessing method");
    println!("💡 Suggestions:");
    println!("   - Ensure the QR code is clearly visible and not blurry");
    println!("   - Try taking a closer screenshot of just the QR code");
    println!("   - Make sure there's good contrast between the QR code and background");
    println!("   - Try option 2 (manual paste) if you can copy the base64 text directly");
    
    Ok(None)
}

// Helper function to enhance contrast
fn enhance_contrast(img: &image::DynamicImage) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
    let gray = img.to_luma8();
    let mut enhanced = gray.clone();
    
    for pixel in enhanced.pixels_mut() {
        let value = pixel.0[0];
        // Simple contrast enhancement: stretch values
        pixel.0[0] = if value < 128 { 
            (value as f32 * 0.5) as u8 
        } else { 
            ((value as f32 - 128.0) * 1.5 + 128.0).min(255.0) as u8 
        };
    }
    
    enhanced
}

// Helper function to invert image
fn invert_image(img: &image::DynamicImage) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
    let gray = img.to_luma8();
    let mut inverted = gray.clone();
    
    for pixel in inverted.pixels_mut() {
        pixel.0[0] = 255 - pixel.0[0];
    }
    
    inverted
}

pub fn scan_qr_chunks_enhanced() -> Vec<u8> {
    let mut result = Vec::new();
    let mut chunk_num = 1;
    
    println!("📥 Starting QR chunk collection...\n");
    
    loop {
        println!("=== Collecting Chunk {} ===", chunk_num);
        let method = get_scan_method();
        
        let decoded_data = match method {
            ScanMethod::File(path) => {
                println!("📁 Scanning image: {}", path);
                match scan_from_file(&path) {
                    Ok(Some(data)) => {
                        println!("✅ QR code found in image!");
                        Some(data)
                    }
                    Ok(None) => {
                        println!("❌ No QR code found in image");
                        None
                    }
                    Err(e) => {
                        println!("❌ Error reading image: {}", e);
                        None
                    }
                }
            }
            
            ScanMethod::Manual => {
                print!("Paste base64 data: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                Some(input.trim().to_string())
            }
        };
        
        if let Some(qr_content) = decoded_data {
            if qr_content.eq_ignore_ascii_case("done") {
                break;
            }
            
            match base64::decode(&qr_content) {
                Ok(decoded) => {
                    result.extend_from_slice(&decoded);
                    println!("✅ Chunk {} added: {} bytes", chunk_num, decoded.len());
                    chunk_num += 1;
                    
                    println!("\nContinue with next chunk? (y/n/done)");
                    let mut response = String::new();
                    io::stdin().read_line(&mut response).unwrap();
                    
                    if response.trim().eq_ignore_ascii_case("n") || 
                       response.trim().eq_ignore_ascii_case("done") {
                        break;
                    }
                }
                Err(_) => {
                    println!("❌ Invalid base64 data. Try again.");
                }
            }
        } else {
            println!("❌ Failed to get QR data. Try again? (y/n)");
            let mut retry = String::new();
            io::stdin().read_line(&mut retry).unwrap();
            
            if retry.trim().eq_ignore_ascii_case("n") {
                break;
            }
        }
    }
    
    println!("📦 Total collected: {} bytes", result.len());
    result
}