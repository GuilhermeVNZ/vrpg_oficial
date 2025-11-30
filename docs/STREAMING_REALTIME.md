# XTTS Real-Time Cinematic Streaming

## Visão Geral

O sistema de streaming real-time cinematográfico do XTTS permite voz contínua sem gaps, com latência inicial de 2.5-4.0s e playback fluido estilo "Critical Role AI".

## Arquitetura

### Thread Architecture

```
Thread A: Qwen 1.5B → Prelude text → Semantic Chunker
Thread B: Qwen 14B → Full narrative → Semantic Chunker
Thread C: XTTS Worker → Generate chunks (adaptive parallel/sequential) → AudioBuffer FIFO
Thread D: Audio Consumer → AudioBuffer FIFO → Native Audio Output (dedicated I/O)
```

**Threads independentes**: Nunca bloqueiam umas às outras.

### Componentes Principais

1. **Semantic Chunker**: Divide texto por pausas narrativas (3-7s, 180-320 chars)
2. **AudioBuffer FIFO**: Thread-safe, Float32 interno, int16 I/O
3. **Pre-Buffer Manager**: Mantém 1-2 chunks à frente (tier-dependent)
4. **XTTS Streaming Worker**: Geração adaptativa (paralela High-End, sequencial Modest)
5. **Native Audio Output**: WASAPI/ASIO/CoreAudio, thread dedicada

## Controle Adaptativo de GPU

### Detecção Automática

O sistema detecta automaticamente o hardware e classifica em tiers:

| Tier | Exemplos | VRAM | Compute | Configuração |
|------|----------|------|---------|--------------|
| **High-End** | RTX 5090, RTX 4090 | 32GB+ | 8.0+ | 2-3 streams, 2.5s buffer, 80-95% GPU |
| **Mid-Range** | RTX 3070, RTX 4060 | 8-16GB | 7.0+ | 1-2 streams, 1.75s buffer, 60-80% GPU |
| **Modest** | RTX 3050, GTX 1660 | 4-8GB | 6.0+ | 1 stream, 1.25s buffer, 40-60% GPU |
| **Low-End** | < 4GB VRAM | < 4GB | < 6.0 | 0-1 stream, 0.75s buffer, 30-50% GPU |

### Configuração Adaptativa

**High-End (RTX 5090)**:
- Paralelização máxima (2-3 CUDA streams)
- Pre-buffer grande (2.5s)
- Alta utilização GPU (80-95%)
- Sem limite de VRAM

**Modest (RTX 3050)**:
- Sequencial apenas (1 CUDA stream)
- Pre-buffer pequeno (1.25s)
- Baixa utilização GPU (40-60%)
- Limite de VRAM (3GB)
- Yield entre chunks (cede GPU)

### Performance Mantida

**Todos os tiers mantêm:**
- Latência inicial: < 5s
- Zero-gap playback
- Sistema responsivo (não sobrecarrega)
- Qualidade RAW preservada

## Otimizações de Áudio

### Sample Rate
- **16-24 kHz**: Suficiente para voz
- **NÃO 48 kHz**: Desperdício de banda e processamento
- **XTTS nativo**: 24 kHz (sem re-amostragem)

### Channels
- **Mono (1 canal)**: Reduz 50% de banda vs estéreo
- **NÃO estéreo**: Dobra banda e processamento desnecessário

### Buffer Size
- **256-512 frames**: Baixa latência
- **NÃO 2048/4096**: Aumenta lag perceptível
- **Testar**: 256, 384, 512 para otimizar

### Formato
- **Float32 interno**: Inferência XTTS (preserva qualidade)
- **int16 I/O**: Eficiente, compatível Opus, reduz 50% banda
- **NÃO float64**: Pesado, desnecessário

### Thread Dedicada
- **I/O de áudio isolada**: Não compartilha com UI/modelo
- **Evita glitches**: Isolamento garante real-time scheduling

## Benchmarks

### Executar Benchmarks

```bash
# Quick benchmark (CI/CD)
cargo test --test benchmark_suite quick_benchmark

# Full benchmark suite
cargo test --test benchmark_suite run_full_benchmark_suite -- --ignored
```

### Configurações Testadas

O benchmark testa 13+ configurações diferentes:
- Diferentes GPU tiers
- Diferentes paralelizações (0-3 streams)
- Diferentes pre-buffer sizes (0.5s - 3.0s)
- Diferentes buffer sizes (256, 384, 512 frames)
- Diferentes sample rates (16kHz, 24kHz)
- Time-stretch optimization

### Identificar Melhor Configuração

O benchmark automaticamente:
1. Testa todas as configurações
2. Compara métricas (latência, RTF, GPU usage)
3. Identifica melhor configuração para menor latência
4. Gera relatório detalhado

## Performance Targets

### Latência
- **High-End**: < 3.8s inicial
- **Mid-Range**: < 4.0s inicial
- **Modest**: < 4.5s inicial
- **Low-End**: < 5.0s inicial

### Real-Time Factor
- **High-End**: < 0.5x (geração 2x mais rápida que playback)
- **Mid-Range**: < 0.6x
- **Modest**: < 0.8x
- **Low-End**: < 1.0x

### GPU Utilization
- **High-End**: 80-95%
- **Mid-Range**: 60-80%
- **Modest**: 40-60%
- **Low-End**: 30-50%

### Qualidade
- **Buffer underrun**: 0 (mandatory)
- **Audio gaps**: 0ms (mandatory)
- **Quality score**: > 0.95 (RAW preserved)

## Configuração Manual

### Environment Variables

```bash
# GPU Control
VRPG_XTTS_GPU_STREAMS=1              # 0=CPU, 1=Sequential, 2-3=Parallel
VRPG_XTTS_GPU_VRAM_LIMIT_MB=3072     # VRAM limit (0=unlimited)
VRPG_XTTS_GPU_UTILIZATION_TARGET=0.6 # Target GPU utilization (0.3-0.95)
VRPG_XTTS_PREBUFFER_SECONDS=1.5      # Pre-buffer size
VRPG_XTTS_PERFORMANCE_PROFILE=auto    # auto|high_performance|balanced|modest
```

### Perfis de Performance

- **`high_performance`**: Máximo GPU, paralelização, buffer grande
- **`balanced`**: Uso moderado, paralelização limitada, buffer médio
- **`modest`**: Mínimo GPU, sequencial, buffer pequeno
- **`auto`**: Detecta hardware automaticamente (padrão)

## Referências

- [STREAMING_REALTIME_SPEC.md](../../rulebook/tasks/implement-tts-service/specs/tts-service/STREAMING_REALTIME_SPEC.md) - Especificação técnica completa
- [CONTROLE_GPU_ADAPTATIVO_SPEC.md](../../rulebook/tasks/implement-tts-service/specs/tts-service/CONTROLE_GPU_ADAPTATIVO_SPEC.md) - Controle adaptativo de GPU
- [AUDIO_OPTIMIZATION_SPEC.md](../../rulebook/tasks/implement-tts-service/specs/tts-service/AUDIO_OPTIMIZATION_SPEC.md) - Otimizações de áudio
- [TEST_SPEC.md](../../rulebook/tasks/implement-tts-service/specs/tts-service/TEST_SPEC.md) - Especificação de testes



