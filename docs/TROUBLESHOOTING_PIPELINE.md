# Guia de Troubleshooting - Pipeline de 3 Agentes

## Problemas Comuns e Soluções

### 1. Pipeline não responde ou congela

#### Sintomas
- Sistema não processa comandos de voz
- Interface não atualiza após comandos
- Logs mostram estado "Processing" por muito tempo

#### Diagnóstico
```bash
# Verificar logs do orquestrador
tail -f logs/orchestrator.log

# Verificar status dos serviços
curl http://localhost:7002/health  # LLM Core
curl http://localhost:7001/health  # ASR
curl http://localhost:7003/health  # TTS
```

#### Soluções
1. **Verificar estado do pipeline**:
   - Pipeline pode estar em estado inválido
   - Reinicie os serviços

2. **Verificar serviços**:
   - Todos os serviços devem estar rodando
   - Verifique portas em conflito

3. **Reset do estado**:
   - Reinicie a sessão ou aplicação
   - O estado será resetado para `WaitingForInput`

### 2. Latência alta nas respostas

#### Sintomas
- Respostas demoram mais de 6 segundos
- Reações do 1.5B demoram mais de 1.2s

#### Diagnóstico
```bash
# Verificar logs de latência
grep "latency" logs/orchestrator.log

# Verificar uso de GPU/CPU
# (use ferramentas do sistema para verificar uso)
```

#### Soluções
1. **Modelos não na GPU**:
   - Verifique se modelos estão sendo carregados na GPU
   - Configure `gpu_layers: -1` no `llm_config.json`

2. **Memória insuficiente**:
   - Feche outros aplicativos
   - Reduza `keep_both_loaded` para `false` se necessário

3. **Contexto muito grande**:
   - Reduza quantidade de eventos na scene context
   - Limpe caches se necessário

### 3. Respostas objetivas demoram

#### Sintomas
- Perguntas simples (HP, AC) demoram para responder
- Respostas que deveriam ser instantâneas demoram

#### Diagnóstico
```bash
# Verificar classificação de intenções
grep "Intent classified" logs/orchestrator.log
```

#### Soluções
1. **Intent Router não classifica corretamente**:
   - Verifique padrões de regex no Intent Router
   - Adicione novos padrões se necessário

2. **Game State Cache não está sendo usado**:
   - Verifique se cache está sendo atualizado
   - Verifique logs de cache hit/miss

### 4. 1.5B não dispara antes do 14B

#### Sintomas
- Respostas longas sem reação inicial
- 14B responde sem prelúdio do 1.5B

#### Diagnóstico
```bash
# Verificar logs de trigger
grep "Triggering 1.5B" logs/orchestrator.log
grep "fast_prelude" logs/orchestrator.log
```

#### Soluções
1. **Trigger não está sendo ativado**:
   - Verifique duração da fala (deve ser >= 6s)
   - Verifique se pause foi detectado
   - Verifique configuração do TriggerCriteria

2. **Estado do pipeline incorreto**:
   - Pipeline pode estar pulando estados
   - Reinicie os serviços

### 5. Erro: "Session file not found"

#### Sintomas
- Não consegue carregar sessão salva
- Erro ao tentar restaurar estado

#### Soluções
1. **Verificar caminho do arquivo**:
   - Arquivo deve estar em `saves/{session_id}.json`
   - Verifique permissões de leitura

2. **Sessão não existe**:
   - Listar sessões disponíveis
   - Verifique se sessão foi salva corretamente

### 6. Erro: "State transition error"

#### Sintomas
- Erro ao tentar transicionar estados
- Pipeline fica em estado inválido

#### Soluções
1. **Transição inválida**:
   - Pipeline só aceita transições válidas
   - Verifique ordem de estados no código

2. **Estado corrompido**:
   - Reinicie o pipeline
   - Estado será resetado para `WaitingForInput`

### 7. Cache não funciona

#### Sintomas
- Mesmas consultas demoram mesmo tempo
- Cache stats mostram muitos misses

#### Soluções
1. **Cache não está sendo atualizado**:
   - Verifique se eventos estão sendo adicionados
   - Verifique limites de histórico

2. **TTL expirando muito rápido**:
   - Ajuste TTL dos caches se necessário
   - Verifique se clean_expired está sendo chamado

### 8. Modelos não carregam

#### Sintomas
- Erro ao iniciar LLM Core
- Modelos não são encontrados

#### Soluções
1. **Caminho incorreto**:
   - Verifique `config/llm_config.json`
   - Verifique se arquivos existem

2. **Permissões**:
   - Verifique permissões de leitura
   - Verifique se arquivos não estão corrompidos

3. **Memória insuficiente**:
   - Libere memória RAM/VRAM
   - Tente carregar um modelo por vez

## Comandos Úteis para Diagnóstico

### Verificar Status dos Serviços

```bash
# LLM Core
curl http://localhost:7002/health

# ASR Service
curl http://localhost:7001/health

# TTS Service
curl http://localhost:7003/health
```

### Verificar Logs

```bash
# Orquestrador
tail -f logs/orchestrator.log

# LLM Core
tail -f logs/llm-core.log

# Filtrar erros
grep -i error logs/*.log
```

### Resetar Estado

```bash
# Parar todos os serviços
pkill -f "llm-core|asr-service|tts-service|orchestrator"

# Limpar caches (se necessário)
rm -rf cache/*

# Reiniciar serviços
# (execute comandos de inicialização)
```

## Relatório de Problemas

Ao reportar um problema, inclua:

1. **Logs relevantes**: Últimas 50 linhas de logs
2. **Configuração**: Conteúdo de `config/llm_config.json`
3. **Hardware**: RAM, VRAM, GPU disponível
4. **Versão**: Versão do VRPG Client
5. **Reprodução**: Passos para reproduzir o problema

## Suporte Adicional

- **Documentação**: [USER_GUIDE_PIPELINE.md](USER_GUIDE_PIPELINE.md)
- **Configuração**: [MODEL_CONFIGURATION_GUIDE.md](MODEL_CONFIGURATION_GUIDE.md)
- **Arquitetura**: [PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md)













