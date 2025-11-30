# Diagnóstico do Pipeline TTS

## Problemas Identificados

### 1. Pronúncia sem pausas, impossível de entender
**Sintoma**: O áudio gerado não tem pausas entre palavras, tornando impossível entender.

**Causa provável**: 
- Phonemização incorreta (phonemes não estão sendo gerados corretamente)
- Falta de pausas entre palavras no mapeamento de phonemes
- Phonemes sendo pulados ou mapeados incorretamente

**Diagnóstico**:
- Phonemizer Python está falhando (espeak não encontrado no PATH do Python)
- Fallback para espeak-ng direto pode não estar gerando pausas corretas

### 2. Voz não é a treinada
**Sintoma**: O áudio gerado não usa a voz do modelo SoVITS treinado.

**Diagnóstico**:
- ✅ Modelo SoVITS está carregado (health endpoint mostra "dungeon_master_en")
- ✅ Modelo existe em disco (598.82 MB, data: 11/24/2025)
- ✅ Config.json existe
- ❌ **SoVITS NÃO está convertendo** - ambos os testes retornaram:
  - Mesma duração (661 ms)
  - Mesmo número de amostras (14592)
  - Isso indica que está usando áudio do Piper diretamente

**Causa provável**:
- Script Python do SoVITS está falhando silenciosamente
- Erro na conversão está sendo ignorado e fazendo fallback para Piper
- Caminho do modelo ou script pode estar incorreto

## Testes Realizados

### Teste 1: Piper isolado (sem SoVITS)
- Actor: `test_piper_only` (não existe, então não usa SoVITS)
- Resultado: ✅ Respondeu
- Duração: 661 ms
- Amostras: 14592

### Teste 2: Pipeline completo (Piper + SoVITS)
- Actor: `dungeon_master_en` (deve usar SoVITS)
- Resultado: ✅ Respondeu
- Duração: 661 ms (MESMA do teste 1)
- Amostras: 14592 (MESMA do teste 1)
- **Conclusão**: SoVITS não está convertendo

## Próximos Passos

1. **Verificar logs do servidor** para erros de SoVITS
2. **Testar script Python diretamente** para ver se funciona
3. **Adicionar mais logs** para diagnosticar onde está falhando
4. **Corrigir phonemização** para adicionar pausas entre palavras
5. **Verificar caminhos** do modelo e script Python

## Logs Adicionados

Foram adicionados logs detalhados em:
- `pipeline.rs`: Logs de início/fim de cada etapa
- `piper.rs`: Logs de phonemização (sucesso/fallback)
- `pipeline.rs`: Logs detalhados da chamada do SoVITS Python

## Arquivos de Teste

- `diagnose_piper_only.wav`: Áudio apenas do Piper (para comparação)
- `diagnose_full_pipeline.wav`: Áudio completo (deve ser diferente se SoVITS funcionar)

