//! Teste XTTS com texto longo para verificar se funciona corretamente

use std::io::Write;
use tts_service::xtts::{SynthesisRequest, XttsModel};

#[tokio::test]
async fn test_xtts_long_text() {
    // Verificar se Coqui TTS está instalado
    let coqui_available = std::process::Command::new("python")
        .arg("-c")
        .arg("from TTS.api import TTS; print('OK')")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !coqui_available {
        println!("⚠️  Coqui TTS not installed. Skipping test.");
        println!("   Install with: pip install TTS");
        return;
    }

    // Criar modelo XTTS com Coqui habilitado
    let mut model = XttsModel::new_with_options(true, false, None); // use_coqui_xtts=true, use_gpu=false
    model
        .load("dummy")
        .await
        .expect("Failed to load XTTS model");

    // Texto longo para teste
    let long_text = r#"In a distant realm where magic flows like rivers and dragons soar through 
clouds of stardust, there lived a brave adventurer named Elara. She had spent years 
training in the ancient arts of combat and spellcasting, preparing for the day when 
she would face the Dark Lord Malachar. The prophecy had foretold that only one with 
pure heart and unwavering courage could defeat the darkness that threatened to consume 
the world. With her trusted companions - a wise wizard named Theron and a fierce 
warrior named Kael - Elara embarked on a perilous journey through enchanted forests, 
across treacherous mountains, and into the depths of forgotten dungeons. Along the way, 
they discovered ancient artifacts of immense power and forged alliances with mystical 
creatures who had long been hidden from mortal eyes. The final battle would test not 
only their strength and skill, but also their bonds of friendship and their faith in 
the light that still remained in the world."#;

    // Criar request
    let request = SynthesisRequest {
        text: long_text.to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.0,
    };

    // Sintetizar
    println!("🎤 Synthesizing long text with XTTS...");
    println!("   Text length: {} characters", long_text.len());
    println!(
        "   Estimated duration: ~{} seconds",
        long_text.len() as f32 / 10.0
    );

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
    println!(
        "   Duration: {:.2} seconds",
        audio.samples.len() as f32 / audio.sample_rate as f32
    );

    // Verificar que temos áudio
    assert!(
        !audio.samples.is_empty(),
        "Audio samples should not be empty"
    );
    assert!(audio.sample_rate > 0, "Sample rate should be > 0");

    // Verificar amplitude
    let max_amplitude = audio.samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
    println!("📊 Max amplitude: {}", max_amplitude);
    assert!(max_amplitude > 0.0, "Audio should have non-zero amplitude");

    // Verificar que o áudio tem duração razoável (pelo menos 10 segundos para texto longo)
    let duration = audio.samples.len() as f32 / audio.sample_rate as f32;
    assert!(
        duration > 10.0,
        "Long text should generate audio longer than 10 seconds"
    );

    // Salvar em arquivo WAV
    let wav_path = "test_xtts_long_text.wav";
    save_wav(&wav_path, &audio.samples, audio.sample_rate, audio.channels)
        .expect("Failed to save WAV file");

    println!("💾 Audio saved to: {}", wav_path);
    println!("✅ Test passed! Check {} to verify audio quality", wav_path);
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
