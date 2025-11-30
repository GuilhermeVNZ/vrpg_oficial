# Ajustes de Qualidade SoVITS - Reduzir Som Robótico

## Parâmetros Otimizados

### Mudanças Aplicadas

1. **`noice_scale`: 0.4 → 0.2**
   - **Efeito**: Valores menores (0.1-0.3) produzem áudio mais natural
   - **Valores maiores** (0.4-0.6) tendem a ser mais robóticos
   - **Recomendado**: 0.1-0.3 para qualidade natural

2. **`nsf_hifigan_enhance`: False → True**
   - **Efeito**: Melhora a qualidade do áudio final
   - **Requer**: Modelo HiFi-GAN pré-treinado (geralmente incluído)
   - **Impacto**: Reduz artefatos e melhora clareza

### Parâmetros Atuais (Otimizados)

```python
{
    "noice_scale": 0.2,           # REDUZIDO (mais natural)
    "f0_predictor": "rmvpe",      # Melhor preditor F0
    "auto_predict_f0": True,      # Auto-detectar F0
    "nsf_hifigan_enhance": True,  # HABILITADO (melhor qualidade)
    "slice_db": -40,              # Threshold de silêncio
    "pad_seconds": 0.5,          # Padding para evitar cortes
}
```

## Testando Diferentes Valores

### noice_scale

- **0.1**: Muito natural, pode perder algumas características da voz
- **0.2**: **RECOMENDADO** - Equilíbrio entre naturalidade e características
- **0.3**: Ainda natural, mais características preservadas
- **0.4**: Mais robótico (valor anterior)
- **0.5+**: Muito robótico, não recomendado

### f0_predictor

- **"rmvpe"**: **RECOMENDADO** - Melhor qualidade geral
- **"fcpe"**: Alternativa de alta qualidade
- **"crepe"**: Boa qualidade, mais lento
- **"pm"**: Rápido, qualidade média
- **"dio"**: Rápido, qualidade menor

## Próximos Ajustes (se ainda estiver robótico)

1. **Reduzir ainda mais `noice_scale`** (0.15 ou 0.1)
2. **Testar `f0_predictor` alternativo** ("fcpe" ou "crepe")
3. **Ajustar `slice_db`** (-35 ou -30 para menos cortes)
4. **Verificar qualidade do modelo treinado** (pode precisar retreinar)

## Comandos para Testar

```powershell
# Teste com parâmetros otimizados
.\test_hello_world_sovits.ps1

# Ou editar diretamente o script Python para testar valores diferentes
# Editar: test_hello_world_sovits.py linha ~239
```

## Resultados Esperados

- ✅ Áudio mais natural e menos robótico
- ✅ Melhor clareza (com enhancer habilitado)
- ✅ Preservação das características da voz do dungeon master
- ⚠️ Pode ser ligeiramente mais lento (enhancer adiciona processamento)

