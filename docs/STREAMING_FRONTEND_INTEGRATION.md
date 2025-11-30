# Streaming Frontend Integration Guide

## Overview

O TTS Service agora suporta streaming de áudio em tempo real via WebSocket e Server-Sent Events (SSE), permitindo que o frontend receba e reproduza áudio conforme ele é gerado, reduzindo significativamente a latência percebida.

## Endpoints

### WebSocket Streaming

**URL**: `ws://localhost:8080/ws/stream`

**Protocolo**: WebSocket binário com mensagens JSON

#### Request (Client → Server)

```json
{
  "text": "This is the text to synthesize",
  "character_id": "dm",
  "language": "en"
}
```

#### Response (Server → Client)

**Audio Chunk**:
```json
{
  "chunk_id": 1,
  "samples": [1234, 5678, ...],
  "sample_rate": 24000,
  "channels": 1
}
```

**Status Message**:
```json
{
  "status": "completed",
  "message": "Streaming completed",
  "buffer_length": null
}
```

**Error Message**:
```json
{
  "status": "error",
  "message": "Error description",
  "buffer_length": null
}
```

### Server-Sent Events (SSE)

**URL**: `POST http://localhost:8080/stream`

**Content-Type**: `application/json`

**Request Body**:
```json
{
  "text": "This is the text to synthesize",
  "character_id": "dm",
  "language": "en"
}
```

**Response**: Stream de eventos SSE com chunks de áudio

### Cancel Streaming

**URL**: `POST http://localhost:8080/stream/cancel`

Cancela o streaming atual e limpa o buffer.

### Get Status

**URL**: `GET http://localhost:8080/stream/status`

Retorna o status atual do streaming:

```json
{
  "status": "Playing",
  "message": null,
  "buffer_length": 2.5
}
```

## Exemplo de Integração (TypeScript/React)

### WebSocket Client

```typescript
import { useEffect, useRef, useState } from 'react';

interface AudioChunkMessage {
  chunk_id: number;
  samples: number[];
  sample_rate: number;
  channels: number;
}

interface StreamingResponse {
  status: string;
  message?: string;
  buffer_length?: number;
}

export function useTTSStreaming() {
  const [isConnected, setIsConnected] = useState(false);
  const [isPlaying, setIsPlaying] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);
  const audioContextRef = useRef<AudioContext | null>(null);
  const sourceNodeRef = useRef<AudioBufferSourceNode | null>(null);

  useEffect(() => {
    // Initialize AudioContext
    audioContextRef.current = new AudioContext({ sampleRate: 24000 });

    return () => {
      if (wsRef.current) {
        wsRef.current.close();
      }
      if (audioContextRef.current) {
        audioContextRef.current.close();
      }
    };
  }, []);

  const connect = () => {
    const ws = new WebSocket('ws://localhost:8080/ws/stream');
    wsRef.current = ws;

    ws.onopen = () => {
      setIsConnected(true);
    };

    ws.onmessage = async (event) => {
      // Check if it's binary (audio chunk) or text (status)
      if (event.data instanceof ArrayBuffer) {
        // Audio chunk
        const text = new TextDecoder().decode(event.data);
        const message: AudioChunkMessage = JSON.parse(text);
        await playAudioChunk(message);
      } else {
        // Status message
        const response: StreamingResponse = JSON.parse(event.data);
        handleStatus(response);
      }
    };

    ws.onerror = (error) => {
      console.error('WebSocket error:', error);
      setIsConnected(false);
    };

    ws.onclose = () => {
      setIsConnected(false);
      setIsPlaying(false);
    };
  };

  const playAudioChunk = async (chunk: AudioChunkMessage) => {
    const audioContext = audioContextRef.current;
    if (!audioContext) return;

    // Convert int16 samples to Float32
    const float32Samples = new Float32Array(chunk.samples.length);
    for (let i = 0; i < chunk.samples.length; i++) {
      float32Samples[i] = chunk.samples[i] / 32768.0;
    }

    // Create AudioBuffer
    const audioBuffer = audioContext.createBuffer(
      chunk.channels,
      float32Samples.length,
      chunk.sample_rate
    );

    // Copy samples to buffer
    audioBuffer.copyToChannel(float32Samples, 0);

    // Create and play source
    const source = audioContext.createBufferSource();
    source.buffer = audioBuffer;
    source.connect(audioContext.destination);
    source.start();

    sourceNodeRef.current = source;
    setIsPlaying(true);
  };

  const handleStatus = (response: StreamingResponse) => {
    if (response.status === 'completed') {
      setIsPlaying(false);
    } else if (response.status === 'error') {
      console.error('Streaming error:', response.message);
      setIsPlaying(false);
    }
  };

  const stream = (text: string, characterId: string, language: string = 'en') => {
    if (!wsRef.current || wsRef.current.readyState !== WebSocket.OPEN) {
      connect();
      // Wait for connection
      setTimeout(() => {
        if (wsRef.current) {
          wsRef.current.send(JSON.stringify({
            text,
            character_id: characterId,
            language,
          }));
        }
      }, 100);
    } else {
      wsRef.current.send(JSON.stringify({
        text,
        character_id: characterId,
        language,
      }));
    }
  };

  const cancel = async () => {
    try {
      await fetch('http://localhost:8080/stream/cancel', {
        method: 'POST',
      });
      setIsPlaying(false);
    } catch (error) {
      console.error('Failed to cancel streaming:', error);
    }
  };

  return {
    isConnected,
    isPlaying,
    stream,
    cancel,
    connect,
  };
}
```

### Uso no Componente

```typescript
function GameComponent() {
  const { stream, cancel, isPlaying } = useTTSStreaming();

  const handleNarrative = (text: string) => {
    stream(text, 'dm', 'en');
  };

  return (
    <div>
      <button onClick={() => handleNarrative('The dragon roars!')}>
        Play Narrative
      </button>
      {isPlaying && (
        <button onClick={cancel}>Cancel</button>
      )}
    </div>
  );
}
```

## Exemplo com SSE (Server-Sent Events)

```typescript
async function streamWithSSE(text: string, characterId: string, language: string) {
  const response = await fetch('http://localhost:8080/stream', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      text,
      character_id: characterId,
      language,
    }),
  });

  const reader = response.body?.getReader();
  const decoder = new TextDecoder();
  const audioContext = new AudioContext({ sampleRate: 24000 });

  if (!reader) return;

  while (true) {
    const { done, value } = await reader.read();
    if (done) break;

    const text = decoder.decode(value);
    const lines = text.split('\n');

    for (const line of lines) {
      if (line.startsWith('data: ')) {
        const data = line.slice(6);
        try {
          const message: AudioChunkMessage = JSON.parse(data);
          await playAudioChunk(message, audioContext);
        } catch (e) {
          // Skip invalid JSON
        }
      }
    }
  }
}
```

## Formato de Áudio

- **Sample Rate**: 24000 Hz (24 kHz)
- **Channels**: Mono (1 canal)
- **Format**: int16 PCM (converter para Float32 no frontend)
- **Buffer Size**: 256-512 frames (configurável)

## Performance

- **Latência Inicial**: 2.5-4.0 segundos (dependendo do tier da GPU)
- **Chunking**: Texto é chunkado semanticamente (3-7 segundos por chunk)
- **Pre-buffering**: 1.5-3.0 segundos antes de iniciar playback
- **Continuidade**: Zero gaps entre chunks

## Tratamento de Erros

```typescript
ws.onerror = (error) => {
  console.error('WebSocket error:', error);
  // Reconectar após delay
  setTimeout(() => connect(), 1000);
};

ws.onclose = (event) => {
  if (event.code !== 1000) {
    // Conexão fechada inesperadamente
    console.warn('Connection closed unexpectedly');
    // Tentar reconectar
    setTimeout(() => connect(), 1000);
  }
};
```

## Cancelamento

Para cancelar o streaming:

```typescript
// Via WebSocket (se implementado no servidor)
ws.send(JSON.stringify({ type: 'cancel' }));

// Via HTTP
await fetch('http://localhost:8080/stream/cancel', {
  method: 'POST',
});
```

## Status e Monitoramento

```typescript
async function getStreamingStatus() {
  const response = await fetch('http://localhost:8080/stream/status');
  const status: StreamingResponse = await response.json();
  
  console.log('Status:', status.status);
  console.log('Buffer length:', status.buffer_length, 'seconds');
}
```

## Notas de Implementação

1. **AudioContext**: Deve ser criado em resposta a uma interação do usuário (click, etc.) para evitar problemas de autoplay.

2. **Buffer Management**: O frontend deve gerenciar o buffer de áudio para garantir reprodução contínua.

3. **Error Recovery**: Implementar reconexão automática em caso de falha na conexão WebSocket.

4. **Performance**: Para melhor performance, use WebSocket ao invés de SSE quando possível.

5. **Format Conversion**: Converter int16 para Float32 antes de criar o AudioBuffer.

## Exemplo Completo (React Hook)

Ver `useTTSStreaming()` acima para um exemplo completo de hook React que gerencia toda a lógica de streaming.



