# Release Notes - Pipeline de 3 Agentes

## Versão 2.0.0 - Pipeline de 3 Agentes

### 🎉 Nova Arquitetura

Esta versão introduz uma **arquitetura completamente nova** com Pipeline de 3 Agentes para melhorar drasticamente a latência e qualidade das respostas.

### ✨ Principais Mudanças

#### Pipeline de 3 Agentes
- **Orquestrador**: Lógica determinística que coordena todos os componentes
- **Qwen-1.5B**: Modelo rápido para reações imediatas (< 1.2s)
- **Qwen-14B**: Modelo completo para narrativa detalhada (< 6s)

#### Otimizações de Latência
- **Respostas Objetivas**: Instantâneas (< 50ms) - sem uso de LLM
- **Regras Simples**: Rápidas (< 1.5s) - apenas Vectorizer + 1.5B
- **Narrativas Completas**: < 6s - com reação inicial do 1.5B

#### Novos Recursos

1. **Intent Router Inteligente**
   - Classifica automaticamente o tipo de pergunta
   - Roteia para o melhor caminho de processamento
   - Cache para perguntas frequentes

2. **Sistema de Caches**
   - **Game State Cache**: Estado do jogo em RAM
   - **Scene Context Cache**: Contexto da cena recente
   - **Lore Cache**: Informações de lore do Vectorizer

3. **Sistema de Persistência de Sessão**
   - Save/Load completo de sessões
   - Preservação de estado entre sessões
   - Versionamento de formato

4. **Respostas Objetivas**
   - Perguntas sobre HP, AC, recursos respondidas diretamente
   - Sem necessidade de LLM para dados factuais
   - Latência ultra-baixa

### 🔧 Mudanças Técnicas

#### Requisitos de Hardware
- **Mínimo**: 16GB RAM, GPU com 8GB VRAM
- **Recomendado**: 32GB RAM, GPU com 16GB+ VRAM
- **Ideal**: 64GB RAM, GPU com 24GB+ VRAM

#### Modelos Necessários
- **Qwen-1.5B**: `qwen2.5-1.5b-instruct-q4_k_m.gguf` (~1GB)
- **Qwen-14B**: `qwen2.5-14b-instruct-q4_k_m.gguf` (~8GB)

Ambos modelos devem estar na pasta:
```
assets-and-models/models/llm/
```

#### Configuração

Novo formato de `config/llm_config.json`:
```json
{
  "models": {
    "1_5b": {
      "path": "assets-and-models/models/llm/qwen2.5-1.5b-instruct-q4_k_m.gguf",
      "max_tokens": 40,
      "temperature": 0.8,
      "top_p": 0.9
    },
    "14b": {
      "path": "assets-and-models/models/llm/qwen2.5-14b-instruct-q4_k_m.gguf",
      "max_tokens": 512,
      "temperature": 0.7,
      "top_p": 0.95
    }
  },
  "memory": {
    "keep_both_loaded": true,
    "preload_on_startup": true
  }
}
```

### 📚 Documentação

Documentação completa disponível:
- **[USER_GUIDE_PIPELINE.md](USER_GUIDE_PIPELINE.md)**: Guia completo para usuários
- **[MODEL_CONFIGURATION_GUIDE.md](MODEL_CONFIGURATION_GUIDE.md)**: Guia de configuração dos modelos
- **[TROUBLESHOOTING_PIPELINE.md](TROUBLESHOOTING_PIPELINE.md)**: Guia de troubleshooting
- **[MIGRATION_GUIDE.md](MIGRATION_GUIDE.md)**: Guia de migração da versão anterior

### 🚀 Migração

Se você está usando uma versão anterior:

1. **Backup de dados**: Faça backup da pasta `saves/` se existir
2. **Baixe modelos**: Você precisa de ambos modelos (1.5B e 14B)
3. **Atualize configuração**: Use o novo formato de `llm_config.json`
4. **Migre sessões**: Sessões antigas serão migradas automaticamente

Consulte **[MIGRATION_GUIDE.md](MIGRATION_GUIDE.md)** para detalhes completos.

### 🐛 Correções

- Correção de latência alta em respostas objetivas
- Melhoria na classificação de intenções
- Otimização de uso de memória com ambos modelos

### 📊 Testes

- **11 testes de persistência**: 100% passando
- **11 testes de integração**: 100% passando
- **8 benchmarks de performance**: Todos dentro dos targets
- **15 testes de regressão**: Nenhuma regressão identificada

### 🔄 Compatibilidade

- **Sessões antigas**: Migração automática suportada
- **Configurações**: Formato antigo ainda funciona, mas novo formato recomendado
- **APIs**: Principais APIs mantêm compatibilidade

### 📝 Notas de Versão

#### Breaking Changes
- Novo formato de `llm_config.json` (formato antigo ainda funciona com aviso)
- Ambos modelos necessários (1.5B e 14B)

#### Deprecations
- Nenhuma depreciação nesta versão

#### Security
- Nenhuma mudança de segurança nesta versão

### 🙏 Agradecimentos

Agradecemos a todos os usuários que testaram e forneceram feedback durante o desenvolvimento desta versão.

### 📞 Suporte

Se encontrar problemas:
1. Consulte **[TROUBLESHOOTING_PIPELINE.md](TROUBLESHOOTING_PIPELINE.md)**
2. Abra uma issue no GitHub
3. Entre em contato via email: suporte@vrpg-client.com

---

**VRPG Client 2.0.0** - Pipeline de 3 Agentes  
*Transformando a experiência de RPG com IA local e tecnologia de ponta!* 🎲✨













