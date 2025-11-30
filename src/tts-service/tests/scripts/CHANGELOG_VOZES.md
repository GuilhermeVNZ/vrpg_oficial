# Changelog - Mudanças nas Vozes

## 2025-11-29

### Mudanças Implementadas

1. **Renomeação de "Mestre Atual" para "Lax Barros"**
   - A voz anteriormente conhecida como "Mestre Atual (narrator_default)" foi renomeada para **"Lax Barros"** (nome do dublador)
   - O arquivo de embedding continua sendo `narrator_default_xtts_reference_clean.wav`
   - O `character_id` foi atualizado de `"narrator_default"` para `"lax_barros"` no código

2. **Ana Florence como Voz do Mestre**
   - A voz original do XTTS "Ana Florence" agora é a voz padrão do **Mestre**
   - O `character_id` do mestre é `"dm"` e não usa embedding customizado (usa voz interna do XTTS)
   - Esta é a voz padrão que será usada quando não houver especificação de voz customizada

### Arquivos Atualizados

#### Código Rust
- `src/tts-service/src/voice_profiles.rs`:
  - `load_default_profiles()`: Agora cria perfil "dm" (Ana Florence) e "lax_barros" (embedding customizado)
  - `auto_discover_xtts_embeddings()`: Mapeia "narrator_default" para "lax_barros" automaticamente

#### Scripts de Benchmark
- `test_benchmark_cpu_vs_gpu.py`: Atualizado para usar "Mestre (Ana Florence)" e "Lax Barros"
- `test_5_voices_benchmark_optimized.py`: Atualizado para usar "Mestre (Ana Florence)" e "Lax Barros"
- `test_5_voices_benchmark.py`: Atualizado para usar "Mestre (Ana Florence)" e "Lax Barros"

#### Testes
- `tests/unit/voice_profiles_test.rs`: Atualizado para testar "dm" e "lax_barros" em vez de "narrator_default"
- Todos os testes agora usam `xtts_embedding_path` em vez de `sovits_model_path`

### Estrutura de Vozes Atual

```
Mestre (character_id: "dm")
├── Voz: Ana Florence (voz original do XTTS)
├── Embedding: Nenhum (usa voz interna)
└── Tipo: DungeonMaster

Lax Barros (character_id: "lax_barros")
├── Voz: Customizada (embedding)
├── Embedding: narrator_default_xtts_reference_clean.wav
└── Tipo: DungeonMaster
```

### Compatibilidade

- O sistema ainda reconhece arquivos com nome `narrator_default_xtts_reference_clean.wav` e os mapeia automaticamente para `lax_barros`
- Código legado que referencia `"narrator_default"` será mapeado para `"lax_barros"` na auto-descoberta
- O `character_id` padrão do mestre mudou de `"narrator_default"` para `"dm"`

### Próximos Passos

1. Atualizar documentação que ainda referencia "narrator_default"
2. Atualizar scripts de configuração que usam o nome antigo
3. Considerar renomear o arquivo físico `narrator_default_xtts_reference_clean.wav` para `lax_barros_xtts_reference_clean.wav` (opcional)



