//! nsfw-scanner-daemon - Native Messaging host for Modesty browser extension.
//!
//! Reads scan requests from stdin (Chrome Native Messaging protocol),
//! runs the Rust scanner (nsfw-lib-rust), writes results to stdout.
//!
//! This is a thin wrapper around nsfw-lib-rust that adds:
//! - Native Messaging protocol (length-prefixed JSON over stdin/stdout)
//! - Model download from GitHub releases
//! - CLI argument parsing (model-dir, no-download)
//!
//! Install as a Chrome Native Messaging host:
//!   Windows: reg add HKCU\Software\Google\Chrome\NativeMessagingHosts\com.modesty.nsfw
//!   Linux:   ~/.config/google-chrome/NativeMessagingHosts/com.modesty.nsfw.json

use clap::Parser;
use nsfw_lib_rust::config::ScannerConfig;
use nsfw_lib_rust::model_download::ensure_models;
use nsfw_lib_rust::native::run_native_messaging;
use nsfw_lib_rust::scanner::create_session;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "nsfw-scanner-daemon")]
#[command(about = "Modesty NSFW scanner Native Messaging host")]
struct Cli {
    /// Directory containing ONNX model files
    #[arg(short, long, default_value = "")]
    model_dir: String,

    /// Skip automatic model download from GitHub
    #[arg(long)]
    no_download: bool,
}

fn main() {
    let cli = Cli::parse();

    let model_dir = if cli.model_dir.is_empty() {
        None
    } else {
        Some(PathBuf::from(cli.model_dir))
    };

    // Build config from CLI args
    let config = ScannerConfig {
        model_dir: model_dir.as_deref().and_then(|p| p.to_str()).unwrap_or("").to_string(),
        auto_download_models: !cli.no_download,
        ..Default::default()
    };

    // Ensure models are available (download if needed)
    if config.auto_download_models {
        if let Err(e) = ensure_models(&config) {
            eprintln!("[nsfw-scanner-daemon] model download failed: {}", e);
            std::process::exit(1);
        }
    }

    // Create scanner session
    let model_dir_str = config.model_dir.as_str();
    let mut session = match create_session(model_dir_str) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[nsfw-scanner-daemon] failed to create scanner session: {}", e);
            std::process::exit(1);
        }
    };

    // Run the Native Messaging loop
    if let Err(e) = run_native_messaging(&mut session, &config) {
        eprintln!("[nsfw-scanner-daemon] native messaging error: {}", e);
        std::process::exit(1);
    }
}