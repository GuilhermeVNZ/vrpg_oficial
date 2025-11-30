//! Teste com stub XTTS (sem Coqui TTS instalado) para verificar estrutura

use std::io::Write;
use tts_service::xtts::{SynthesisRequest, XttsModel};

#[tokio::test]
async fn test_hello_world_xtts_stub() {
    // Criar modelo XTTS SEM Coqui (usa stub)
    let mut model = XttsModel::new_with_options(false, false, None); // use_coqui_xtts=false
    model
        .load("dummy")
        .await
        .expect("Failed to load XTTS model");

    // Criar request
    let request = SynthesisRequest {
        text: "Hello World".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.0,
    };

    // Sintetizar
    println!("🎤 Synthesizing 'Hello World' with XTTS stub...");
    let audio = model
        .synthesize(&request)
        .await
        .expect("Failed to synthesize");

    println!(
        "✅ Audio generated: {} samples, {} Hz, {} channels",
        audio.samples.len(),
        audio.sample_rate,
        audio.channels
    );

    // Verificar que temos áudio
    assert!(
        !audio.samples.is_empty(),
        "Audio samples should not be empty"
    );
    assert!(audio.sample_rate > 0, "Sample rate should be > 0");

    // Verificar amplitude (deve ter algum sinal)
    let max_amplitude = audio.samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
    println!("📊 Max amplitude: {}", max_amplitude);
    assert!(max_amplitude > 0.0, "Audio should have non-zero amplitude");

    // Salvar em arquivo WAV para verificação manual
    let wav_path = "test_hello_world_xtts_stub.wav";
    save_wav(&wav_path, &audio.samples, audio.sample_rate, audio.channels)
        .expect("Failed to save WAV file");

    println!("💾 Audio saved to: {}", wav_path);
    println!("✅ Test passed! Check {} to verify audio quality", wav_path);
    println!("ℹ️  Note: This is stub audio. Install Coqui TTS for real synthesis.");
}

fn save_wav(path: &str, samples: &[f32], sample_rate: u32, channels: u16) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;

    // WAV header
    file.write_all(b"RIFF")?;
    let data_size = (samples.len() * 2 + 36) as u32;
    file.write_all(&data_size.to_le_bytes())?;
    file.write_all(b"WAVE")?;
    file.write_all(b"fmt ")?;
    file.write_all(&16u32.to_le_bytes())?; // fmt chunk size
    file.write_all(&1u16.to_le_bytes())?; // PCM format
    file.write_all(&channels.to_le_bytes())?;
    file.write_all(&sample_rate.to_le_bytes())?;
    let byte_rate = sample_rate * channels as u32 * 2;
    file.write_all(&byte_rate.to_le_bytes())?;
    file.write_all(&(channels * 2).to_le_bytes())?; // block align
    file.write_all(&16u16.to_le_bytes())?; // bits per sample
    file.write_all(b"data")?;
    let sample_data_size = (samples.len() * 2) as u32;
    file.write_all(&sample_data_size.to_le_bytes())?;

    // Convert f32 samples to i16
    for &sample in samples {
        let i16_sample = (sample.clamp(-1.0, 1.0) * 32767.0) as i16;
        file.write_all(&i16_sample.to_le_bytes())?;
    }

    Ok(())
}
