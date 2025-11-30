# Configuração da API Key do Google AI Studio (Nano Banana Pro)

## Resposta Rápida

**SIM**, a chave gerada no Google AI Studio pode ser usada fora do Google AI Studio, incluindo no Cursor e em qualquer aplicação que faça chamadas REST à API do Google.

## Requisitos

1. **Faturamento Ativado**: O modelo Nano Banana Pro (Imagen 4.0) requer faturamento ativado no Google Cloud
2. **Chave de API Válida**: A chave deve ser gerada no [Google AI Studio](https://aistudio.google.com)
3. **Configuração Correta**: A chave deve ser configurada como variável de ambiente

## Configuração Passo a Passo

### 1. Obter a Chave de API

1. Acesse [Google AI Studio](https://aistudio.google.com)
2. Faça login com sua conta Google
3. No menu lateral, clique em "Obter chave de API" (Get API Key)
4. Crie uma nova chave ou use uma existente
5. **IMPORTANTE**: Certifique-se de que o faturamento está ativado para usar o Nano Banana Pro

### 2. Configurar no Projeto

#### Opção A: Arquivo `.env` (Recomendado)

1. No diretório `vrpg-client`, crie ou edite o arquivo `.env`
2. Adicione a linha:

```bash
GEMINI_API_KEY=sua-chave-aqui
```

**Exemplo completo** (baseado em `env.example`):

```bash
# Google AI (for additional services)
GOOGLE_API_KEY=sua-chave-aqui
GEMINI_API_KEY=sua-chave-aqui
```

#### Opção B: Variável de Ambiente do Sistema

**Windows (PowerShell):**
```powershell
$env:GEMINI_API_KEY="sua-chave-aqui"
```

**Windows (CMD):**
```cmd
set GEMINI_API_KEY=sua-chave-aqui
```

**Linux/Mac:**
```bash
export GEMINI_API_KEY=sua-chave-aqui
```

### 3. Verificar a Configuração

Execute um dos scripts de teste:

```bash
# Testar geração de imagens
node test_gemini_image.mjs

# Listar modelos disponíveis
node list_models.mjs
```

## Modelos Disponíveis

### Imagen (Geração de Imagens)

- `imagen-4.0-ultra-generate-preview-06-06` - Nano Banana Pro Ultra (✅ **FUNCIONA** - use este)
- `imagen-4.0-generate-preview-06-06` - Nano Banana Pro (⚠️ retorna resposta vazia)
- `nano-banana-pro-preview` - Nome alternativo (❌ não funciona com :predict)
- `imagen-3.0-generate-001` - Imagen 3.0

### Gemini (LLM)

- `gemini-2.0-flash-exp` - Gemini 2.0 Flash (experimental)
- `gemini-2.5-flash` - Gemini 2.5 Flash
- `gemini-2.5-pro` - Gemini 2.5 Pro
- `gemini-1.5-pro` - Gemini 1.5 Pro
- `gemini-1.5-flash` - Gemini 1.5 Flash

## Endpoints da API

### Geração de Imagens (Imagen)

```javascript
// Endpoint para Imagen 4.0 Ultra (Nano Banana Pro - RECOMENDADO)
const url = `https://generativelanguage.googleapis.com/v1beta/models/imagen-4.0-ultra-generate-preview-06-06:predict?key=${apiKey}`;

// Endpoint para Imagen 4.0 (retorna resposta vazia - NÃO USAR)
// const url = `https://generativelanguage.googleapis.com/v1beta/models/imagen-4.0-generate-preview-06-06:predict?key=${apiKey}`;

// Endpoint para Imagen 3.0
const url = `https://generativelanguage.googleapis.com/v1beta/models/imagen-3.0-generate-001:predict?key=${apiKey}`;
```

### Geração de Conteúdo (Gemini LLM)

```javascript
// Endpoint para Gemini
const url = `https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key=${apiKey}`;
```

## Exemplo de Uso

Veja os arquivos de exemplo no projeto:

- `generate_goblin_sprites.mjs` - Geração de sprites com Imagen 4.0
- `test_gemini_image.mjs` - Teste de geração de imagens
- `list_models.mjs` - Listar modelos disponíveis

## Problemas Comuns

### 1. Erro: "API key not found"

**Solução**: Verifique se o arquivo `.env` existe e contém `GEMINI_API_KEY=...`

### 2. Erro: "Billing required" ou "Quota exceeded"

**Solução**: 
- Ative o faturamento no Google Cloud Console
- Verifique se há créditos disponíveis
- O Nano Banana Pro requer conta paga

### 3. Erro: "Model not found" ou Resposta Vazia

**Solução**: 
- Use `imagen-4.0-ultra-generate-preview-06-06` ao invés de `imagen-4.0-generate-preview-06-06`
- O modelo `imagen-4.0-generate-preview-06-06` retorna resposta vazia `{}`
- O modelo `nano-banana-pro-preview` não funciona com o endpoint `:predict`
- Use `list_models.mjs` ou `test_api_key.mjs` para ver modelos disponíveis

### 4. Erro: "Invalid API key"

**Solução**:
- Verifique se a chave está completa (sem espaços extras)
- Gere uma nova chave no Google AI Studio
- Certifique-se de que a chave não expirou

### 5. Resposta vazia ou sem dados

**Solução**:
- Verifique a estrutura da resposta da API
- Alguns endpoints retornam `predictions`, outros `images`
- Veja o código em `generate_goblin_sprites.mjs` para exemplo de parsing

## Estrutura de Resposta

### Imagen API (predict endpoint)

```json
{
  "predictions": [
    {
      "bytesBase64Encoded": "iVBORw0KGgoAAAANSUhEUgAA..."
    }
  ]
}
```

### Gemini API (generateContent endpoint)

```json
{
  "candidates": [
    {
      "content": {
        "parts": [
          { "text": "..." }
        ]
      }
    }
  ]
}
```

## Custos

- **Imagen 4.0 (Nano Banana Pro)**: ~$0.134 por imagem 1K/2K, ~$0.24 por imagem 4K
- **Gemini 2.0 Flash**: $0.50/$1.50 por 1M tokens (input/output)
- **Gemini 2.5 Pro**: $1.25/$5.00 por 1M tokens (input/output)

**Importante**: Monitore o uso no [Google Cloud Console](https://console.cloud.google.com) para evitar custos inesperados.

## Segurança

⚠️ **NUNCA** commite o arquivo `.env` no Git!

O arquivo `.env` já está no `.gitignore`, mas sempre verifique antes de fazer commit:

```bash
git check-ignore .env  # Deve retornar ".env"
```

## Referências

- [Google AI Studio](https://aistudio.google.com)
- [Google Cloud Console](https://console.cloud.google.com)
- [Documentação da API Gemini](https://ai.google.dev/docs)
- [Documentação da API Imagen](https://cloud.google.com/vertex-ai/docs/generative-ai/image/overview)

