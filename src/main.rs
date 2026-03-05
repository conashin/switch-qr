use std::env;
use std::fs::File;
use std::io::Read;

use chrono::Local;
use image::Luma;
use qrcode::QrCode;
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
#[allow(non_snake_case)]
struct OutputData {
    名稱: String,
    IP: String,
    DNS: String,
    備註: String,
    購入年: String,
    QRCode建立日期: String,
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

    // Create OutputData
    let output_data = OutputData {
        名稱: input_data.name.clone(),
        IP: input_data.ip,
        DNS: input_data.dns,
        備註: input_data.notes,
        購入年: input_data.purchase_year,
        QRCode建立日期: current_date,
    };

    // Serialize OutputData to JSON string
    let json_string = serde_json::to_string(&output_data)?;

    // Generate QR Code
    let code = QrCode::new(json_string.as_bytes())?;
    let image = code.render::<Luma<u8>>().build();

    // Save as PNG
    let output_filename = format!("{}.png", input_data.name);
    image.save(&output_filename)?;

    println!("Successfully generated QR code: {}", output_filename);

    Ok(())
}
