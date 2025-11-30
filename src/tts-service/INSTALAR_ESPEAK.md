# Instalar espeak-ng para Phonemização

O Piper TTS requer espeak-ng para converter texto em phonemes. Sem ele, o TTS usará um fallback simplificado.

## Windows

### Opção 1: Winget (Recomendado)
```powershell
winget install espeak-ng
```

### Opção 2: Chocolatey
```powershell
choco install espeak-ng
```

### Opção 3: Download Manual
1. Acesse: https://github.com/espeak-ng/espeak-ng/releases
2. Baixe o instalador Windows
3. Instale e adicione ao PATH

### Opção 4: Scoop
```powershell
scoop install espeak-ng
```

## Linux

### Debian/Ubuntu
```bash
sudo apt-get update
sudo apt-get install espeak-ng
```

### Fedora
```bash
sudo dnf install espeak-ng
```

### Arch Linux
```bash
sudo pacman -S espeak-ng
```

## macOS

### Homebrew
```bash
brew install espeak-ng
```

## Verificar Instalação

Após instalar, verifique se está funcionando:

```bash
espeak-ng --version
```

Ou no Windows:
```powershell
espeak --version
```

## Nota

Se espeak-ng não estiver instalado, o TTS service usará um fallback simplificado que pode não ter a mesma qualidade de phonemização, mas ainda funcionará.

