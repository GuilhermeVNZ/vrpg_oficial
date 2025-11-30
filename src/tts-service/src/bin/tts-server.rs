use clap::Parser;
use std::env;
use tts_service::TtsServer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "3002")]
    port: u16,

    #[arg(short, long)]
    base_path: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    // Calculate base path: if not provided, try to find it relative to executable
    let base_path = if let Some(custom_path) = args.base_path {
        custom_path
    } else if let Ok(env_path) = env::var("TTSSERVICE_BASE_PATH") {
        env_path
    } else {
        // Try to find assets-and-models relative to executable
        // Executable is in target/release/, so we need to go up 3 levels
        let exe_path = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| std::path::PathBuf::from("."));

        // Try multiple relative paths
        let possible_paths = vec![
            exe_path.join("../../assets-and-models/models/tts"),
            exe_path.join("../../../assets-and-models/models/tts"),
            std::path::PathBuf::from("../../assets-and-models/models/tts"),
            std::path::PathBuf::from("G:/vrpg/vrpg-client/assets-and-models/models/tts"),
        ];

        possible_paths
            .iter()
            .find(|p| p.exists())
            .and_then(|p| p.to_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "../../assets-and-models/models/tts".to_string())
    };

    tracing::info!("Starting TTS Service on port {}", args.port);
    tracing::info!("Base path: {}", base_path);

    let server = TtsServer::new(&base_path)?;

    // Initialize voice profiles (load XTTS embeddings)
    tracing::info!("Initializing voice profiles from: {}", base_path);
    if let Err(e) = server.initialize_voice_profiles().await {
        tracing::warn!("Failed to initialize voice profiles: {}", e);
        tracing::warn!("Continuing without voice profiles (will use default voices)");
    } else {
        tracing::info!("Voice profiles initialized successfully");
    }

    // XTTS models are loaded automatically on first use
    tracing::info!("XTTS will be loaded automatically on first synthesis request");

    tracing::info!("TTS Service ready! Listening on port {}", args.port);
    server.start(args.port).await?;

    Ok(())
}
