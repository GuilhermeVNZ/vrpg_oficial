# Scripts de Automação VRPG Client

Este diretório contém scripts PowerShell para executar operações comuns de forma isolada e segura, evitando problemas de parsing e execução no terminal integrado.

## Por que usar esses scripts?

Os scripts PowerShell aqui foram criados para:
- **Isolar execução**: Cada script roda em seu próprio contexto, evitando problemas de parsing
- **Tratamento de erros**: Cada script verifica códigos de saída e para na primeira falha
- **Feedback claro**: Mensagens coloridas indicam o progresso e resultados
- **Reutilização**: Podem ser chamados tanto manualmente quanto via npm scripts

## Scripts Disponíveis

### `check-quality.ps1`
Executa todas as verificações de qualidade do projeto:
- Rust format check (`cargo fmt --check`)
- Clippy linting (`cargo clippy`)
- TypeScript type checking (`npm run type-check`)
- ESLint (`npm run lint`)

**Uso:**
```powershell
.\scripts\check-quality.ps1
```

ou via npm:
```bash
npm run check:quality
```

### `run-tests.ps1`
Executa todos os testes do projeto:
- Testes Rust (`cargo test`)
- Testes TypeScript (`npm test`)

**Uso:**
```powershell
.\scripts\run-tests.ps1
```

ou via npm:
```bash
npm run test:all
```

### `fix-warnings.ps1`
Corrige automaticamente warnings que podem ser corrigidos:
- Auto-formata Rust (`cargo fmt`)
- Auto-corrige Clippy (`cargo clippy --fix`)
- Auto-corrige ESLint (`npm run lint -- --fix`)

**Uso:**
```powershell
.\scripts\fix-warnings.ps1
```

ou via npm:
```bash
npm run fix:warnings
```

## Estratégia Recomendada

Para evitar problemas com comandos complexos no terminal integrado:

1. **Use os scripts PowerShell** para operações complexas ou que envolvem múltiplos comandos
2. **Use comandos simples** diretamente no terminal para operações únicas e rápidas
3. **Verifique resultados** antes de continuar para a próxima etapa

## Exemplo de Workflow

```powershell
# 1. Fazer mudanças no código
# ... editar arquivos ...

# 2. Auto-corrigir warnings
npm run fix:warnings

# 3. Verificar qualidade
npm run check:quality

# 4. Executar testes
npm run test:all

# 5. Se tudo passar, continuar com commit
```

## Troubleshooting

### Erro: "Execution Policy"
Se você receber um erro sobre Execution Policy, execute:
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### Erro: "Script não encontrado"
Certifique-se de estar no diretório raiz do projeto (`vrpg-client`) ao executar os scripts.

### Erro: "Comando não reconhecido"
Certifique-se de que:
- Rust/Cargo está instalado e no PATH
- Node.js/npm está instalado e no PATH
- Você executou `npm install` no projeto

