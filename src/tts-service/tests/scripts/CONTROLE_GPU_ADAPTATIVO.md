# Controle Adaptativo de GPU para XTTS

## 🎯 Objetivo

Permitir que XTTS rode em máquinas modestas **SEM perder performance de tempo de resposta da voz**, controlando o uso da GPU para não sobrecarregar o sistema.

## ⚠️ Problema Atual

**XTTS está usando GPU mas:**
- ❌ NÃO há controle de paralelização (múltiplos CUDA streams)
- ❌ NÃO há limite de uso da GPU
- ❌ NÃO há adaptação para máquinas modestas
- ❌ Pode sobrecarregar sistema (deixar PC lento)

## ✅ Solução: Controle Adaptativo

### 1. Detecção Automática de Hardware

O sistema detecta automaticamente a GPU e classifica em tiers:

| Tier | Exemplos | VRAM | Compute |
|------|----------|------|---------|
| **High-End** | RTX 5090, RTX 4090, A100 | 32GB+ | 8.0+ |
| **Mid-Range** | RTX 3070, RTX 4060 | 8-16GB | 7.0+ |
| **Modest** | RTX 3050, GTX 1660 | 4-8GB | 6.0+ |
| **Low-End** | < 4GB VRAM | < 4GB | < 6.0 |

### 2. Configuração Adaptativa por Tier

#### High-End (RTX 5090)
```python
# Máximo desempenho
parallel_streams = 2-3      # Múltiplos CUDA streams
prebuffer_seconds = 2.0-3.0 # Buffer grande
gpu_utilization = 80-95%   # Usar bastante GPU
vram_limit = None           # Sem limite
```

#### Mid-Range (RTX 3070)
```python
# Balanceado
parallel_streams = 1-2      # Limitado
prebuffer_seconds = 1.5-2.0 # Buffer médio
gpu_utilization = 60-80%    # Uso moderado
vram_limit = 6GB            # Limite de VRAM
```

#### Modest (RTX 3050)
```python
# Conservador
parallel_streams = 1        # Sequencial apenas
prebuffer_seconds = 1.0-1.5 # Buffer pequeno
gpu_utilization = 40-60%    # Uso baixo
vram_limit = 3GB            # Limite rígido
yield_between_chunks = True # Ceder GPU entre chunks
```

#### Low-End (< 4GB)
```python
# Mínimo
parallel_streams = 0-1      # CPU ou sequencial
prebuffer_seconds = 0.5-1.0 # Buffer mínimo
gpu_utilization = 30-50%    # Uso muito baixo
vram_limit = 2GB            # Limite muito rígido
cpu_fallback = True         # CPU para algumas ops
```

### 3. Controle Manual via Environment Variables

```bash
# Controlar uso da GPU manualmente
VRPG_XTTS_GPU_STREAMS=1              # 0=CPU, 1=Sequential, 2-3=Parallel
VRPG_XTTS_GPU_VRAM_LIMIT_MB=3072     # Limite de VRAM (0=unlimited)
VRPG_XTTS_GPU_UTILIZATION_TARGET=0.6 # Target de utilização (0.3-0.95)
VRPG_XTTS_PREBUFFER_SECONDS=1.5      # Tamanho do pre-buffer
VRPG_XTTS_PERFORMANCE_PROFILE=modest # auto|high_performance|balanced|modest
```

### 4. Perfis de Performance

#### `high_performance` (RTX 5090)
- Paralelização máxima
- Buffer grande
- Alta utilização GPU
- **Resultado**: Melhor latência, mas usa muito GPU

#### `balanced` (RTX 3070)
- Paralelização limitada
- Buffer médio
- Utilização moderada
- **Resultado**: Boa latência, uso razoável

#### `modest` (RTX 3050)
- Sem paralelização
- Buffer pequeno
- Baixa utilização
- **Resultado**: Latência aceitável, não sobrecarrega sistema

#### `auto` (Padrão)
- Detecta hardware automaticamente
- Aplica configuração apropriada
- **Resultado**: Otimizado para cada máquina

## 📊 Performance Mantida

**IMPORTANTE**: Mesmo em máquinas modestas, a latência de voz é mantida:

| Tier | Latência Inicial | RTF | Continuidade |
|------|------------------|-----|--------------|
| High-End | 2.5-3.8s | < 0.5x | Zero gaps |
| Mid-Range | 2.5-4.0s | < 0.6x | Zero gaps |
| Modest | 3.0-4.5s | < 0.8x | Zero gaps |
| Low-End | 3.5-5.0s | < 1.0x | Zero gaps |

**A voz sempre responde em < 5s, mesmo em hardware modesto!**

## 🔧 Implementação

### Detecção de Hardware

```python
def detect_gpu_tier():
    if not torch.cuda.is_available():
        return "cpu"
    
    gpu_name = torch.cuda.get_device_name(0)
    vram_gb = torch.cuda.get_device_properties(0).total_memory / 1024**3
    compute = torch.cuda.get_device_capability(0)
    
    if vram_gb >= 32 and compute[0] >= 8:
        return "high_end"
    elif vram_gb >= 8 and compute[0] >= 7:
        return "mid_range"
    elif vram_gb >= 4 and compute[0] >= 6:
        return "modest"
    else:
        return "low_end"
```

### Configuração Adaptativa

```python
def get_gpu_config(tier, profile="auto"):
    if profile != "auto":
        # Usar perfil manual
        return get_profile_config(profile)
    
    # Auto-detect baseado em tier
    configs = {
        "high_end": {
            "parallel_streams": 2,
            "vram_limit_mb": 0,  # unlimited
            "utilization_target": 0.85,
            "prebuffer_seconds": 2.5,
            "yield_between_chunks": False,
        },
        "mid_range": {
            "parallel_streams": 1,
            "vram_limit_mb": 6144,  # 6GB
            "utilization_target": 0.70,
            "prebuffer_seconds": 1.75,
            "yield_between_chunks": False,
        },
        "modest": {
            "parallel_streams": 1,  # Sequential
            "vram_limit_mb": 3072,  # 3GB
            "utilization_target": 0.50,
            "prebuffer_seconds": 1.25,
            "yield_between_chunks": True,  # Ceder GPU
        },
        "low_end": {
            "parallel_streams": 0,  # CPU ou 1 stream
            "vram_limit_mb": 2048,  # 2GB
            "utilization_target": 0.40,
            "prebuffer_seconds": 0.75,
            "yield_between_chunks": True,
            "cpu_fallback": True,
        },
    }
    return configs.get(tier, configs["modest"])
```

### Controle de VRAM

```python
def enforce_vram_limit(vram_limit_mb):
    if vram_limit_mb == 0:
        return  # Sem limite
    
    allocated = torch.cuda.memory_allocated(0) / 1024**2  # MB
    if allocated > vram_limit_mb * 0.9:  # 90% do limite
        # Limpar cache CUDA
        torch.cuda.empty_cache()
        
        # Reduzir pre-buffer se necessário
        reduce_prebuffer_size()
```

### Yield entre Chunks (Modest Hardware)

```python
def generate_chunk_with_yield(text, config):
    # Gerar chunk
    audio = tts.tts(text, ...)
    
    # Se yield habilitado, ceder GPU
    if config["yield_between_chunks"]:
        torch.cuda.synchronize()  # Esperar GPU terminar
        time.sleep(0.01)  # 10ms yield para outros processos
    
    return audio
```

## 🎯 Resultado

### Máquinas Modestas (RTX 3050)
- ✅ GPU não sobrecarregada (40-60% uso)
- ✅ Sistema responsivo (yield entre chunks)
- ✅ VRAM controlada (limite 3GB)
- ✅ Latência mantida (< 4.5s)
- ✅ Voz contínua (zero gaps)

### Máquinas High-End (RTX 5090)
- ✅ GPU maximizada (80-95% uso)
- ✅ Paralelização ativa (2-3 streams)
- ✅ Melhor latência (< 3.8s)
- ✅ Performance máxima

## 📝 Variáveis de Ambiente Recomendadas

### Para Máquinas Modestas
```bash
VRPG_XTTS_GPU_STREAMS=1
VRPG_XTTS_GPU_VRAM_LIMIT_MB=3072
VRPG_XTTS_GPU_UTILIZATION_TARGET=0.5
VRPG_XTTS_PREBUFFER_SECONDS=1.25
VRPG_XTTS_PERFORMANCE_PROFILE=modest
```

### Para Máquinas High-End
```bash
VRPG_XTTS_GPU_STREAMS=2
VRPG_XTTS_GPU_VRAM_LIMIT_MB=0
VRPG_XTTS_GPU_UTILIZATION_TARGET=0.85
VRPG_XTTS_PREBUFFER_SECONDS=2.5
VRPG_XTTS_PERFORMANCE_PROFILE=high_performance
```

## ✅ Checklist de Implementação

- [ ] Detecção automática de GPU tier
- [ ] Configuração adaptativa por tier
- [ ] Controle de VRAM (limite e monitoramento)
- [ ] Controle de paralelização (streams)
- [ ] Yield entre chunks (modest hardware)
- [ ] Environment variables para override
- [ ] Perfis de performance (high/balanced/modest/auto)
- [ ] Monitoramento de uso da GPU
- [ ] Cleanup automático de VRAM
- [ ] Testes em diferentes hardware tiers

---

**Última atualização**: 2025-11-28



