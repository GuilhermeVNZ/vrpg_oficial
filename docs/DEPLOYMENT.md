# VRPG Client - Deploy e Distribuição

## Visão Geral

O VRPG Client é distribuído como uma aplicação Electron multiplataforma, com suporte completo para Windows, macOS e Linux. O sistema de build automatizado gera instaladores nativos e portáveis para cada plataforma.

## Electron App Architecture

O VRPG Client é empacotado via **electron-builder** com serviços Rust embedded:

### Serviços Embedded
- **llm_srv**: Serviço LLM local (Pipeline: Qwen 1.5B + Qwen 14B)
- **asr_srv**: Serviço de reconhecimento de voz (Whisper-large-v3-turbo)
- **tts_srv**: Serviço de síntese de voz (XTTS v2 ONNX)
- **rules_srv**: Motor de regras D&D 5e em Rust
- **memory_srv**: Serviço de memória com integração Hive

### Distribuição de Modelos

**Models shipped inside `./assets-and-models/models/`**:
```
assets-and-models/models/
├── llm/
│   ├── qwen2.5-1.5b-instruct-q4_k_m.gguf  # Mestre Reflexo (reação rápida)
│   └── qwen2.5-14b-instruct-q4_k_m.gguf   # Mestre Real (narrativa completa)
├── asr/
│   └── whisper-large-v3-turbo.bin
└── tts/
    └── xtts_v2.onnx
```

**Arquitetura de Pipeline**: O sistema usa um pipeline de 2 modelos:
- **Qwen 1.5B**: Reação humana imediata, prelúdio emocional (< 1.2s)
- **Qwen 14B**: Narrativa completa, consequências, resolução (< 6s)

**Regra de Ouro**: O 1.5B sempre responde antes do 14B para evitar silêncio cognitivo.

Todos os modelos necessários são incluídos no pacote da aplicação, garantindo execução 100% offline.

## Arquitetura de Deploy

### Componentes de Distribuição
```
vrpg-client-release/
├── installers/
│   ├── windows/
│   │   ├── VRPG-Client-Setup-1.0.0.exe     # NSIS Installer
│   │   └── VRPG-Client-1.0.0-portable.exe  # Portable
│   ├── macos/
│   │   ├── VRPG-Client-1.0.0.dmg           # DMG Installer
│   │   └── VRPG-Client-1.0.0-mac.zip       # Portable
│   └── linux/
│       ├── VRPG-Client-1.0.0.AppImage      # AppImage
│       ├── vrpg-client_1.0.0_amd64.deb     # Debian Package
│       └── vrpg-client-1.0.0.x86_64.rpm    # RPM Package
├── checksums.txt                           # SHA256 checksums
└── release-notes.md                        # Release notes
```

### Requisitos de Sistema

#### Mínimos
- **CPU**: Intel i5-8400 / AMD Ryzen 5 2600 (6 cores)
- **RAM**: 8GB DDR4
- **GPU**: DirectX 11 / OpenGL 4.1 compatible (GPU com 8GB+ VRAM recomendado para pipeline de 2 modelos)
- **Storage**: 18GB espaço livre (SSD recomendado) - inclui Qwen 1.5B (~1GB) + Qwen 14B (~8.2GB) + outros modelos
- **OS**: Windows 10 1909+ / macOS 10.15+ / Ubuntu 20.04+

#### Recomendados
- **CPU**: Intel i7-10700K / AMD Ryzen 7 3700X (8+ cores)
- **RAM**: 16GB DDR4
- **GPU**: NVIDIA GTX 1660 / AMD RX 580 (16GB+ VRAM recomendado para pipeline de 2 modelos com aceleração GPU)
- **Storage**: 28GB espaço livre (NVMe SSD) - inclui ambos os modelos LLM + assets
- **OS**: Windows 11 / macOS 12+ / Ubuntu 22.04+

## Build System

### Electron Builder Configuration
```json
{
  "build": {
    "appId": "com.vrpg.client",
    "productName": "VRPG Client",
    "directories": {
      "output": "dist",
      "buildResources": "build-resources"
    },
    
    "files": [
      "dist-electron/**/*",
      "dist-client/**/*",
      "assets/**/*",
      "config/**/*",
      "!src/**/*",
      "!tests/**/*",
      "!*.md"
    ],
    
    "extraResources": [
      {
        "from": "assets/models",
        "to": "models",
        "filter": ["**/*"]
      },
      {
        "from": "binaries/${os}",
        "to": "bin",
        "filter": ["**/*"]
      }
    ],
    
    "win": {
      "target": [
        {
          "target": "nsis",
          "arch": ["x64"]
        },
        {
          "target": "portable",
          "arch": ["x64"]
        }
      ],
      "icon": "build-resources/icon.ico",
      "requestedExecutionLevel": "asInvoker"
    },
    
    "mac": {
      "target": [
        {
          "target": "dmg",
          "arch": ["x64", "arm64"]
        },
        {
          "target": "zip",
          "arch": ["x64", "arm64"]
        }
      ],
      "icon": "build-resources/icon.icns",
      "category": "public.app-category.games",
      "hardenedRuntime": true,
      "entitlements": "build-resources/entitlements.mac.plist"
    },
    
    "linux": {
      "target": [
        {
          "target": "AppImage",
          "arch": ["x64"]
        },
        {
          "target": "deb",
          "arch": ["x64"]
        },
        {
          "target": "rpm",
          "arch": ["x64"]
        }
      ],
      "icon": "build-resources/icon.png",
      "category": "Game"
    },
    
    "nsis": {
      "oneClick": false,
      "allowToChangeInstallationDirectory": true,
      "createDesktopShortcut": true,
      "createStartMenuShortcut": true,
      "shortcutName": "VRPG Client"
    },
    
    "dmg": {
      "title": "VRPG Client ${version}",
      "backgroundColor": "#000000",
      "window": {
        "width": 600,
        "height": 400
      }
    }
  }
}
```

### Cross-Platform Build Scripts
```json
{
  "scripts": {
    "build": "npm run build:client && npm run build:services && npm run build:electron",
    "build:client": "vite build",
    "build:services": "cargo build --release",
    "build:electron": "electron-builder build",
    
    "dist": "npm run build && electron-builder",
    "dist:win": "npm run build && electron-builder --win",
    "dist:mac": "npm run build && electron-builder --mac",
    "dist:linux": "npm run build && electron-builder --linux",
    
    "pack": "npm run build && electron-builder --dir",
    "pack:all": "npm run build && electron-builder --win --mac --linux --dir",
    
    "release": "npm run test && npm run dist && npm run upload:release"
  }
}
```

## Pipeline de CI/CD

### GitHub Actions Workflow
```yaml
# .github/workflows/release.yml
name: Release Build

on:
  push:
    tags:
      - 'v*'

jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18
          cache: 'npm'
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: x86_64-pc-windows-msvc
      
      - name: Install dependencies
        run: npm ci
      
      - name: Download models
        run: npm run download:models
        env:
          MODEL_CACHE_KEY: ${{ secrets.MODEL_CACHE_KEY }}
      
      - name: Build Rust services
        run: cargo build --release --target x86_64-pc-windows-msvc
      
      - name: Build Electron app
        run: npm run dist:win
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          CSC_LINK: ${{ secrets.WIN_CSC_LINK }}
          CSC_KEY_PASSWORD: ${{ secrets.WIN_CSC_KEY_PASSWORD }}
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: windows-build
          path: dist/*.exe

  build-macos:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18
          cache: 'npm'
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: x86_64-apple-darwin
      
      - name: Add ARM64 target
        run: rustup target add aarch64-apple-darwin
      
      - name: Install dependencies
        run: npm ci
      
      - name: Download models
        run: npm run download:models
      
      - name: Build Rust services (x64)
        run: cargo build --release --target x86_64-apple-darwin
      
      - name: Build Rust services (ARM64)
        run: cargo build --release --target aarch64-apple-darwin
      
      - name: Create universal binaries
        run: npm run create:universal-binaries
      
      - name: Build Electron app
        run: npm run dist:mac
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          CSC_LINK: ${{ secrets.MAC_CSC_LINK }}
          CSC_KEY_PASSWORD: ${{ secrets.MAC_CSC_KEY_PASSWORD }}
          APPLE_ID: ${{ secrets.APPLE_ID }}
          APPLE_ID_PASSWORD: ${{ secrets.APPLE_ID_PASSWORD }}
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: macos-build
          path: dist/*.{dmg,zip}

  build-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18
          cache: 'npm'
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install system dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y libasound2-dev libssl-dev pkg-config
      
      - name: Install dependencies
        run: npm ci
      
      - name: Download models
        run: npm run download:models
      
      - name: Build Rust services
        run: cargo build --release
      
      - name: Build Electron app
        run: npm run dist:linux
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: linux-build
          path: dist/*.{AppImage,deb,rpm}

  create-release:
    needs: [build-windows, build-macos, build-linux]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Download all artifacts
        uses: actions/download-artifact@v3
      
      - name: Generate checksums
        run: |
          cd windows-build && sha256sum * > ../checksums-windows.txt
          cd ../macos-build && sha256sum * > ../checksums-macos.txt
          cd ../linux-build && sha256sum * > ../checksums-linux.txt
          cat checksums-*.txt > checksums.txt
      
      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            windows-build/*
            macos-build/*
            linux-build/*
            checksums.txt
          generate_release_notes: true
          draft: false
          prerelease: ${{ contains(github.ref, 'alpha') || contains(github.ref, 'beta') }}
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Gerenciamento de Assets

### Model Download System
```typescript
// scripts/download-models.ts
import { createHash } from 'crypto';
import { createWriteStream } from 'fs';
import { pipeline } from 'stream/promises';
import { fetch } from 'undici';

interface ModelConfig {
  name: string;
  url: string;
  path: string;
  checksum: string;
  size: number;
}

class ModelDownloader {
  private config: ModelConfig[];
  
  constructor(configPath: string) {
    this.config = JSON.parse(fs.readFileSync(configPath, 'utf8'));
  }
  
  async downloadAll(): Promise<void> {
    console.log('Downloading AI models...');
    
    for (const model of this.config) {
      await this.downloadModel(model);
    }
    
    console.log('All models downloaded successfully!');
  }
  
  private async downloadModel(model: ModelConfig): Promise<void> {
    const targetPath = path.join('assets/models', model.path);
    
    // Check if model already exists and is valid
    if (await this.validateModel(targetPath, model.checksum)) {
      console.log(`✓ ${model.name} already exists and is valid`);
      return;
    }
    
    console.log(`⬇ Downloading ${model.name} (${this.formatSize(model.size)})...`);
    
    const response = await fetch(model.url);
    if (!response.ok) {
      throw new Error(`Failed to download ${model.name}: ${response.statusText}`);
    }
    
    // Ensure directory exists
    await fs.promises.mkdir(path.dirname(targetPath), { recursive: true });
    
    // Download with progress
    const fileStream = createWriteStream(targetPath);
    await pipeline(response.body!, fileStream);
    
    // Validate checksum
    if (!(await this.validateModel(targetPath, model.checksum))) {
      throw new Error(`Checksum validation failed for ${model.name}`);
    }
    
    console.log(`✓ ${model.name} downloaded and validated`);
  }
  
  private async validateModel(filePath: string, expectedChecksum: string): Promise<boolean> {
    try {
      const fileBuffer = await fs.promises.readFile(filePath);
      const hash = createHash('sha256').update(fileBuffer).digest('hex');
      return hash === expectedChecksum;
    } catch {
      return false;
    }
  }
  
  private formatSize(bytes: number): string {
    const sizes = ['B', 'KB', 'MB', 'GB'];
    if (bytes === 0) return '0 B';
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return Math.round(bytes / Math.pow(1024, i) * 100) / 100 + ' ' + sizes[i];
  }
}

// CLI usage
if (require.main === module) {
  const downloader = new ModelDownloader('config/models.json');
  downloader.downloadAll().catch(console.error);
}
```

### Asset Optimization
```javascript
// scripts/optimize-assets.js
const sharp = require('sharp');
const path = require('path');
const fs = require('fs').promises;

async function optimizeImages() {
  const imageDir = 'assets/images';
  const outputDir = 'assets/images/optimized';
  
  await fs.mkdir(outputDir, { recursive: true });
  
  const files = await fs.readdir(imageDir, { recursive: true });
  const imageFiles = files.filter(f => /\.(png|jpg|jpeg)$/i.test(f));
  
  for (const file of imageFiles) {
    const inputPath = path.join(imageDir, file);
    const outputPath = path.join(outputDir, file.replace(/\.(png|jpg|jpeg)$/i, '.webp'));
    
    await sharp(inputPath)
      .webp({ quality: 85 })
      .toFile(outputPath);
    
    console.log(`Optimized: ${file} -> ${path.basename(outputPath)}`);
  }
}

async function optimizeAudio() {
  // Convert audio files to optimized formats
  const audioDir = 'assets/audio';
  const files = await fs.readdir(audioDir, { recursive: true });
  const audioFiles = files.filter(f => /\.(wav|mp3|flac)$/i.test(f));
  
  for (const file of audioFiles) {
    // Use ffmpeg to convert to OGG Vorbis
    const inputPath = path.join(audioDir, file);
    const outputPath = inputPath.replace(/\.(wav|mp3|flac)$/i, '.ogg');
    
    await execAsync(`ffmpeg -i "${inputPath}" -c:a libvorbis -q:a 4 "${outputPath}"`);
    console.log(`Converted: ${file} -> ${path.basename(outputPath)}`);
  }
}

optimizeImages().then(() => optimizeAudio()).catch(console.error);
```

## Code Signing e Notarização

### Windows Code Signing
```powershell
# scripts/sign-windows.ps1
param(
    [Parameter(Mandatory=$true)]
    [string]$CertificatePath,
    
    [Parameter(Mandatory=$true)]
    [string]$CertificatePassword,
    
    [Parameter(Mandatory=$true)]
    [string]$ExecutablePath
)

# Sign the executable
& "C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64\signtool.exe" sign `
    /f $CertificatePath `
    /p $CertificatePassword `
    /t http://timestamp.digicert.com `
    /fd SHA256 `
    /v $ExecutablePath

if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ Successfully signed $ExecutablePath"
} else {
    Write-Error "✗ Failed to sign $ExecutablePath"
    exit 1
}
```

### macOS Notarization
```bash
#!/bin/bash
# scripts/notarize-macos.sh

APP_PATH="$1"
APPLE_ID="$2"
APPLE_ID_PASSWORD="$3"
TEAM_ID="$4"

echo "Notarizing $APP_PATH..."

# Upload for notarization
xcrun notarytool submit "$APP_PATH" \
    --apple-id "$APPLE_ID" \
    --password "$APPLE_ID_PASSWORD" \
    --team-id "$TEAM_ID" \
    --wait

if [ $? -eq 0 ]; then
    echo "✓ Notarization successful"
    
    # Staple the notarization
    xcrun stapler staple "$APP_PATH"
    
    if [ $? -eq 0 ]; then
        echo "✓ Stapling successful"
    else
        echo "✗ Stapling failed"
        exit 1
    fi
else
    echo "✗ Notarization failed"
    exit 1
fi
```

## Auto-Update System

### Update Server Configuration
```typescript
// src/main/updater.ts
import { autoUpdater } from 'electron-updater';
import { dialog } from 'electron';

export class AppUpdater {
  constructor() {
    autoUpdater.checkForUpdatesAndNotify();
    this.setupEventHandlers();
  }
  
  private setupEventHandlers(): void {
    autoUpdater.on('checking-for-update', () => {
      console.log('Checking for update...');
    });
    
    autoUpdater.on('update-available', (info) => {
      console.log('Update available:', info.version);
      this.showUpdateDialog(info);
    });
    
    autoUpdater.on('update-not-available', () => {
      console.log('Update not available');
    });
    
    autoUpdater.on('error', (err) => {
      console.error('Update error:', err);
    });
    
    autoUpdater.on('download-progress', (progressObj) => {
      const percent = Math.round(progressObj.percent);
      console.log(`Download progress: ${percent}%`);
    });
    
    autoUpdater.on('update-downloaded', () => {
      console.log('Update downloaded');
      this.showRestartDialog();
    });
  }
  
  private async showUpdateDialog(info: any): Promise<void> {
    const result = await dialog.showMessageBox({
      type: 'info',
      title: 'Update Available',
      message: `VRPG Client ${info.version} is available`,
      detail: 'Would you like to download and install it now?',
      buttons: ['Download', 'Later'],
      defaultId: 0
    });
    
    if (result.response === 0) {
      autoUpdater.downloadUpdate();
    }
  }
  
  private async showRestartDialog(): Promise<void> {
    const result = await dialog.showMessageBox({
      type: 'info',
      title: 'Update Ready',
      message: 'Update has been downloaded',
      detail: 'Restart the application to apply the update',
      buttons: ['Restart Now', 'Later'],
      defaultId: 0
    });
    
    if (result.response === 0) {
      autoUpdater.quitAndInstall();
    }
  }
}
```

### Update Server (Express.js)
```typescript
// update-server/server.ts
import express from 'express';
import { readFileSync } from 'fs';
import { join } from 'path';

const app = express();
const PORT = process.env.PORT || 3000;

interface UpdateInfo {
  version: string;
  releaseDate: string;
  url: string;
  sha512: string;
  size: number;
}

app.get('/update/:platform/:version', (req, res) => {
  const { platform, version } = req.params;
  
  try {
    const updateInfo = getLatestUpdate(platform);
    
    if (isNewerVersion(updateInfo.version, version)) {
      res.json(updateInfo);
    } else {
      res.status(204).send(); // No update available
    }
  } catch (error) {
    res.status(500).json({ error: 'Failed to check for updates' });
  }
});

function getLatestUpdate(platform: string): UpdateInfo {
  const updatePath = join(__dirname, 'updates', platform, 'latest.json');
  return JSON.parse(readFileSync(updatePath, 'utf8'));
}

function isNewerVersion(latest: string, current: string): boolean {
  const latestParts = latest.split('.').map(Number);
  const currentParts = current.split('.').map(Number);
  
  for (let i = 0; i < Math.max(latestParts.length, currentParts.length); i++) {
    const latestPart = latestParts[i] || 0;
    const currentPart = currentParts[i] || 0;
    
    if (latestPart > currentPart) return true;
    if (latestPart < currentPart) return false;
  }
  
  return false;
}

app.listen(PORT, () => {
  console.log(`Update server running on port ${PORT}`);
});
```

## Monitoramento e Analytics

### Crash Reporting
```typescript
// src/main/crash-reporter.ts
import { crashReporter } from 'electron';
import { app } from 'electron';

export function setupCrashReporting(): void {
  crashReporter.start({
    productName: 'VRPG Client',
    companyName: 'VRPG Team',
    submitURL: 'https://crash-reports.vrpg-client.com/submit',
    uploadToServer: true,
    extra: {
      version: app.getVersion(),
      platform: process.platform,
      arch: process.arch
    }
  });
}
```

### Usage Analytics
```typescript
// src/main/analytics.ts
import { app, ipcMain } from 'electron';

interface AnalyticsEvent {
  event: string;
  properties: Record<string, any>;
  timestamp: number;
  sessionId: string;
}

export class Analytics {
  private sessionId: string;
  private events: AnalyticsEvent[] = [];
  
  constructor() {
    this.sessionId = this.generateSessionId();
    this.setupEventHandlers();
  }
  
  private setupEventHandlers(): void {
    app.on('ready', () => {
      this.track('app_started', {
        version: app.getVersion(),
        platform: process.platform
      });
    });
    
    app.on('before-quit', () => {
      this.track('app_closed');
      this.flush();
    });
    
    ipcMain.handle('analytics:track', (_, event: string, properties: any) => {
      this.track(event, properties);
    });
  }
  
  track(event: string, properties: Record<string, any> = {}): void {
    this.events.push({
      event,
      properties,
      timestamp: Date.now(),
      sessionId: this.sessionId
    });
    
    // Flush every 50 events or every 5 minutes
    if (this.events.length >= 50) {
      this.flush();
    }
  }
  
  private async flush(): Promise<void> {
    if (this.events.length === 0) return;
    
    try {
      await fetch('https://analytics.vrpg-client.com/events', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ events: this.events })
      });
      
      this.events = [];
    } catch (error) {
      console.error('Failed to send analytics:', error);
    }
  }
  
  private generateSessionId(): string {
    return Math.random().toString(36).substring(2) + Date.now().toString(36);
  }
}
```

## Deployment Checklist

### Pre-Release
- [ ] All tests passing (unit, integration, E2E)
- [ ] Performance benchmarks met
- [ ] Security audit completed
- [ ] Code signing certificates valid
- [ ] Models downloaded and validated
- [ ] Assets optimized
- [ ] Documentation updated
- [ ] Release notes prepared

### Build Process
- [ ] Clean build environment
- [ ] Dependencies updated
- [ ] Cross-platform builds successful
- [ ] Installers generated
- [ ] Code signing completed
- [ ] Checksums generated
- [ ] Upload to distribution channels

### Post-Release
- [ ] Release published
- [ ] Update server configured
- [ ] Monitoring systems active
- [ ] User feedback channels ready
- [ ] Support documentation updated
- [ ] Community notifications sent

---

Este sistema de deploy garante distribuição confiável, segura e automatizada do VRPG Client em todas as plataformas suportadas, com atualizações automáticas e monitoramento completo.
