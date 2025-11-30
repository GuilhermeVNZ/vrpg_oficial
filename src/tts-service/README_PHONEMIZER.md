# Instalação do Phonemizer

O script `phonemize_for_piper.py` requer a biblioteca `phonemizer` do Python, que é a mesma usada pelo Piper Python.

## Instalação

### Opção 1: Instalar globalmente
```bash
pip install phonemizer
```

### Opção 2: Instalar no ambiente virtual do SoVITS
Se você já tem um ambiente virtual do SoVITS configurado:

```bash
# Ativar o ambiente virtual do SoVITS
cd G:\vrpg\vrpg-client\assets-and-models\models\tts\sovits
.\venv310\Scripts\activate  # Windows
# ou
source venv310/bin/activate  # Linux/Mac

# Instalar phonemizer
pip install phonemizer
```

### Opção 3: Criar ambiente virtual separado para TTS
```bash
cd G:\vrpg\vrpg-client\src\tts-service
python -m venv venv
.\venv\Scripts\activate  # Windows
pip install phonemizer
```

## Verificação

Teste se está funcionando:
```bash
python scripts/phonemize_for_piper.py "Hello world" -l en-us
```

Deve retornar uma lista de phonemes IPA.

## Dependências

O `phonemizer` requer:
- Python 3.7+
- espeak-ng (já instalado anteriormente)
- Segments (instalado automaticamente com phonemizer)

## Nota

O script Rust tentará usar o script Python primeiro. Se falhar, fará fallback para espeak-ng direto (menos preciso, mas funciona).

