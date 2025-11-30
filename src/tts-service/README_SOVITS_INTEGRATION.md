# Integração SoVITS Python

O serviço TTS agora usa o modelo SoVITS Python real para converter áudio do Piper.

## Requisitos

1. **Modelo SoVITS treinado**: O modelo `dungeon_master_en.pth` deve estar em:
   ```
   G:\vrpg\vrpg-client\assets-and-models\models\tts\sovits\dungeon_master_en.pth
   ```

2. **Configuração**: O arquivo `config.json` deve estar em:
   ```
   G:\vrpg\vrpg-client\assets-and-models\models\tts\sovits\config.json
   ```

3. **Ambiente Python**: O script precisa acessar o ambiente virtual do SoVITS ou ter as dependências instaladas:
   - soundfile
   - numpy
   - torch
   - inference.infer_tool (do SoVITS)

## Como funciona

1. O Rust gera áudio neutro com Piper
2. Salva o áudio em um arquivo WAV temporário
3. Chama o script Python `sovits_convert.py`
4. O script Python carrega o modelo SoVITS e converte o áudio
5. O Rust lê o áudio convertido e retorna

## Configuração do Speaker

O speaker padrão é `dungeon_master_en`. Para verificar speakers disponíveis:

```python
from inference.infer_tool import Svc
svc = Svc("dungeon_master_en.pth", "config.json")
print(svc.spk2id.keys())
```

## Troubleshooting

### Erro: "SoVITS Python script not found"
- Verifique se o script está em `src/tts-service/scripts/sovits_convert.py`

### Erro: "Dependências não encontradas"
- Ative o ambiente virtual do SoVITS antes de iniciar o serviço
- Ou instale as dependências globalmente

### Erro: "Speaker não encontrado"
- Verifique se o speaker `dungeon_master_en` existe no `config.json`
- Verifique o nome do speaker no modelo treinado

### Áudio não convertido (usa Piper direto)
- Verifique os logs do servidor
- O sistema faz fallback para Piper se SoVITS falhar
- Verifique se o modelo está carregado corretamente

