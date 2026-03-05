use std::env;
use std::fs::File;
use std::io::Read;

use chrono::Local;
use image::Luma;
use qrcode::{EcLevel, QrCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct InputData {
    name: String,
    ip: String,
    dns: String,
    #[serde(default)]
    notes: String,
    purchase_year: String,
}

#[derive(Debug, Serialize)]
struct OutputData {
    #[serde(rename = "N")]
    name: String,
    #[serde(rename = "IP")]
    ip: String,
    #[serde(rename = "DNS")]
    dns: String,
    #[serde(rename = "M")]
    notes: String,
    #[serde(rename = "Y")]
    purchase_year: String,
    #[serde(rename = "D")]
    qrcode_date: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the input file path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input.yaml>", args[0]);
        std::process::exit(1);
    }
    let input_path = &args[1];

    // Read the YAML file
    let mut file = File::open(input_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse YAML to InputData
    let input_data: InputData = serde_yaml::from_str(&contents)?;

    // Get current date
    let current_date = Local::now().format("%Y-%m-%d").to_string();

    // Create OutputData with minimized keys for smaller QR code payload
    let output_data = OutputData {
        name: input_data.name.clone(),
        ip: input_data.ip,
        dns: input_data.dns,
        notes: input_data.notes,
        purchase_year: input_data.purchase_year,
        qrcode_date: current_date,
    };

    // Serialize OutputData to minified JSON string (no pretty formatting)
    let json_string = serde_json::to_string(&output_data)?;

    // Generate QR Code with Low error correction (L) to minimize matrix complexity
    let code = QrCode::with_error_correction_level(json_string.as_bytes(), EcLevel::L)?;

    // Render the image without the default white quiet zone border
    // This maximizes the size of the individual black/white modules when printed
    let image = code.render::<Luma<u8>>()
        .quiet_zone(false)
        .build();

    // Save as PNG
    let output_filename = format!("{}.png", input_data.name);
    image.save(&output_filename)?;

    println!("Successfully generated optimized QR code: {}", output_filename);
    println!("Payload size: {} bytes", json_string.len());

    Ok(())
}
