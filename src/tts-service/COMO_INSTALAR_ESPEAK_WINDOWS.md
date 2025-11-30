# Como Instalar espeak-ng no Windows

## Opção 1: Instalador Automático (Mais Fácil) ✅

Se você baixou o instalador `.exe`:
1. Execute o arquivo `.exe`
2. Siga o assistente de instalação
3. O instalador adiciona automaticamente ao PATH
4. **Pronto!** Não precisa fazer mais nada

## Opção 2: ZIP Portable (Manual)

Se você baixou o arquivo `.zip`:

### Passo 1: Escolher Local

**Recomendado:** Salve em um destes locais:

```
C:\Program Files\espeak-ng\
```

OU dentro do projeto:

```
G:\vrpg\vrpg-client\tools\espeak-ng\
```

### Passo 2: Extrair

1. Extraia todo o conteúdo do ZIP para a pasta escolhida
2. Deve ter um arquivo `espeak-ng.exe` (ou `espeak.exe`) dentro

### Passo 3: Adicionar ao PATH

#### Método A: Via Interface Gráfica (Recomendado)

1. Pressione `Win + R`
2. Digite: `sysdm.cpl` e pressione Enter
3. Vá na aba **"Avançado"**
4. Clique em **"Variáveis de Ambiente"**
5. Em **"Variáveis do sistema"**, encontre **"Path"**
6. Clique em **"Editar"**
7. Clique em **"Novo"**
8. Adicione o caminho completo (ex: `C:\Program Files\espeak-ng`)
9. Clique em **"OK"** em todas as janelas
10. **Reinicie o terminal/PowerShell** para aplicar

#### Método B: Via PowerShell (Administrador)

```powershell
# Adicionar ao PATH do sistema (permanente)
[Environment]::SetEnvironmentVariable(
    "Path",
    [Environment]::GetEnvironmentVariable("Path", "Machine") + ";C:\Program Files\espeak-ng",
    "Machine"
)
```

**OU** se salvou no projeto:

```powershell
[Environment]::SetEnvironmentVariable(
    "Path",
    [Environment]::GetEnvironmentVariable("Path", "Machine") + ";G:\vrpg\vrpg-client\tools\espeak-ng",
    "Machine"
)
```

### Passo 4: Verificar

Feche e abra um novo PowerShell, depois execute:

```powershell
espeak-ng --version
```

Ou:

```powershell
espeak --version
```

Se mostrar a versão, está funcionando! ✅

## Opção 3: Usar Caminho Absoluto (Sem PATH)

Se você não quiser adicionar ao PATH, podemos modificar o código para usar o caminho completo. Me diga onde você salvou e eu ajusto o código!

## Verificação Rápida

Execute este comando para verificar se está funcionando:

```powershell
espeak-ng -q -x --phonout=- --phonout-ipa -v en-us "Hello world"
```

Se mostrar phonemes IPA, está funcionando perfeitamente!

