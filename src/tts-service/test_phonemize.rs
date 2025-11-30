use tts_service::phonemizer;

fn main() {
    let text = "Hello world";
    let language = "en";
    
    match phonemizer::phonemize(text, language) {
        Ok(phonemes) => {
            println!("SUCESSO! Phonemes gerados: {:?}", phonemes);
            println!("Total: {} phonemes", phonemes.len());
        }
        Err(e) => {
            println!("ERRO: {}", e);
        }
    }
}
