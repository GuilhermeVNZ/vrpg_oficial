# Explicação: Fallback para CPU no ONNX Runtime

## Situação Atual

Os logs mostram muitas mensagens como:
```
Force fallback to CPU execution for node: ... because the CPU execution path is deemed faster than overhead involved with execution on other EPs
```

## Isso é Normal?

**SIM!** Isso é comportamento esperado do ONNX Runtime.

### Por que isso acontece?

1. **Overhead de Transferência**: Transferir dados entre CPU e GPU tem um custo. Para operações pequenas (como `Gather`, `Slice`, `Concat`), esse overhead pode ser maior que o benefício de usar GPU.

2. **Otimização Automática**: O ONNX Runtime analisa cada nó do modelo e decide automaticamente qual execution provider usar baseado em:
   - Tamanho da operação
   - Overhead de transferência CPU↔GPU
   - Complexidade computacional
   - Disponibilidade de operadores na GPU

3. **Modelo Piper**: O modelo Piper TTS tem muitas operações pequenas de manipulação de dados (slicing, gathering, concatenation) que são naturalmente mais rápidas em CPU.

## O que está rodando em GPU?

Mesmo com muitos fallbacks para CPU, as operações **computacionalmente intensivas** ainda rodam em GPU:

- ✅ **Convoluções** (`Conv`, `ConvTranspose`) - rodam em GPU
- ✅ **Multiplicações de Matrizes** (`MatMul`, `Gemm`) - rodam em GPU
- ✅ **Operações de Rede Neural** (camadas densas, attention) - rodam em GPU
- ✅ **Operações com cuDNN** - todas rodam em GPU

## Evidências de que GPU está funcionando:

1. **cuDNN ativo**: `cuDNN version: 91600` nos logs
2. **CUDA registrado**: `Successfully registered CUDAExecutionProvider`
3. **Memcpy nodes**: `28 Memcpy nodes are added to the graph` - isso indica transferências CPU↔GPU, o que significa que GPU está sendo usada
4. **Alocação de VRAM**: Logs mostram `Extending BFCArena for Cuda` - memória sendo alocada na GPU

## Performance

- **Primeira inferência**: ~3.9 segundos (carregamento inicial + inferência)
- **Inferências subsequentes**: Muito mais rápidas (cache, modelo já carregado)

O tempo de 3.9s na primeira inferência inclui:
- Carregamento do modelo na GPU
- Alocação de memória VRAM
- Transferência de dados CPU→GPU
- Execução mista (CPU + GPU)

## Como melhorar?

### Opção 1: Aceitar o comportamento atual (Recomendado)
O ONNX Runtime está otimizando automaticamente. As operações que realmente se beneficiam de GPU já estão rodando em GPU.

### Opção 2: Forçar mais operações na GPU (Não recomendado)
Isso pode **piorar** a performance porque:
- Aumenta overhead de transferência
- Operações pequenas são mais lentas em GPU
- Pode causar mais memcpy nodes

### Opção 3: Usar um modelo diferente
Alguns modelos ONNX são mais "GPU-friendly" que outros. O Piper foi otimizado para ser eficiente em CPU e GPU misto.

## Conclusão

**O GPU está funcionando corretamente!** O fallback para CPU é uma **otimização automática** do ONNX Runtime, não um problema. As operações computacionalmente intensivas estão rodando em GPU, e as operações pequenas estão rodando em CPU onde são mais rápidas.

## Verificação

Para confirmar que GPU está sendo usada:
1. Execute `nvidia-smi -l 1` durante uma síntese
2. Observe o aumento no uso de GPU (utilization.gpu)
3. Observe o aumento no uso de VRAM (memory.used)

Se você ver aumento durante a síntese, a GPU está sendo usada! ✅



