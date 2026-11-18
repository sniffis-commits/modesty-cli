//! nsfw-scan-rust - Rust CLI scanner matching Python nsfw-scan behavior.
//!
//! Usage:
//!   nsfw-scan-rust <image> [--json] [--details] [--model-dir PATH] [--no-download]
//!   nsfw-scan-rust --batch <image1> <image2> ... [--json]
//!
//! Matches Python nsfw-scan output exactly.

use clap::Parser;
use image::DynamicImage;
use nsfw_lib_rust::scanner::{create_session, scan_image_file};
use nsfw_lib_rust::config::ScannerConfig;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "nsfw-scan-rust")]
#[command(about = "Modesty NSFW scanner Rust CLI - matches Python nsfw-scan")]
struct Cli {
    /// Image file(s) to scan
    #[arg(value_name = "IMAGE")]
    images: Vec<PathBuf>,

    /// Output as JSON
    #[arg(short, long)]
    json: bool,

    /// Include detailed scores (nsfw_score, gender scores, classes)
    #[arg(long)]
    details: bool,

    /// Directory containing ONNX model files
    #[arg(short = 'm', long, default_value = "")]
    model_dir: String,

    /// Skip automatic model download from GitHub
    #[arg(long)]
    no_download: bool,

    /// Process multiple images as a batch (requires --json)
    #[arg(long)]
    batch: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.images.is_empty() {
        eprintln!("Error: at least one image required");
        std::process::exit(1);
    }

    if cli.batch && !cli.json {
        eprintln!("Error: --batch requires --json");
        std::process::exit(1);
    }

    // Build config
    let config = ScannerConfig {
        model_dir: if cli.model_dir.is_empty() { String::new() } else { cli.model_dir },
        auto_download_models: !cli.no_download,
        ..Default::default()
    };

    // Create scanner session
    let model_dir_str = if config.model_dir.is_empty() { "" } else { &config.model_dir };
    let mut session = match create_session(model_dir_str) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to create scanner session: {}", e);
            std::process::exit(1);
        }
    };

    // Scan images
    #[derive(serde::Serialize)]
    #[serde(untagged)]
    enum ScanOutput {
        Result(nsfw_lib_rust::scanner::ScanResult),
        Error { error: String },
    }

    let mut results = Vec::new();
    for img_path in &cli.images {
        match scan_image_file(&mut session, img_path.to_str().unwrap_or("")) {
            Ok(result) => {
                if cli.json || cli.batch {
                    results.push((img_path.to_string_lossy().to_string(), ScanOutput::Result(result)));
                } else {
                    print_human(&result);
                }
            }
            Err(e) => {
                if cli.json || cli.batch {
                    results.push((img_path.to_string_lossy().to_string(), ScanOutput::Error { error: e.to_string() }));
                } else {
                    eprintln!("Error scanning {}: {}", img_path.display(), e);
                    std::process::exit(1);
                }
            }
        }
    }

    if cli.json || cli.batch {
        let output = if cli.batch {
            serde_json::json!({ "results": results })
        } else {
            serde_json::json!(results[0].1)
        };
        if cli.details {
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        } else {
            println!("{}", serde_json::to_string(&output).unwrap());
        }
    }
}

fn print_human(result: &nsfw_lib_rust::scanner::ScanResult) {
    if let Some(rating) = result.rating {
        let verdict = result.verdict.as_deref().unwrap_or("unknown");
        println!("rating: {} ({})", rating, verdict);
    } else {
        println!("rating: not assessed");
    }

    if let Some(ref phrase) = result.phrase {
        println!("phrase: {}", phrase);
    }
}