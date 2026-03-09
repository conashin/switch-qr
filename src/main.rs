use std::env;
use std::fs::File;
use std::io::Read;

use qrqrpar::{EcLevel, QrCode, QrStyle, RmqrStrategy};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct InputData {
    name: String,
    url: String,
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

    // Create a customized style for the rMQR code (no quiet zone)
    let style = QrStyle {
        quiet_zone: 0.0,
        ..Default::default()
    };

    // Generate rMQR Code
    let code = QrCode::rmqr_with_options(input_data.url.as_bytes(), EcLevel::M, RmqrStrategy::Area)?;

    // Save as PNG
    let output_filename = format!("{}.png", input_data.name);
    code.save_png(&output_filename, &style)?;

    println!("Successfully generated rMQR code: {}", output_filename);
    println!("Payload size: {} bytes", input_data.url.len());

    Ok(())
}
