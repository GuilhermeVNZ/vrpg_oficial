#!/usr/bin/env node
/**
 * Teste específico para o Nano Banana Pro (Imagen 4.0)
 * 
 * Uso: node test_nano_banana.mjs
 */

import fs from 'fs';

async function testNanoBanana() {
    console.log('='.repeat(70));
    console.log('Teste do Nano Banana Pro (Imagen 4.0)');
    console.log('='.repeat(70));
    console.log();

    // Ler API Key
    let apiKey = '';
    try {
        const envContent = fs.readFileSync('.env', 'utf8');
        const match = envContent.match(/GEMINI_API_KEY=(.*)/);
        if (match) {
            apiKey = match[1].trim().replace(/^["']|["']$/g, '');
        }
    } catch (e) {
        console.error('❌ Erro ao ler .env:', e.message);
        process.exit(1);
    }

    if (!apiKey) {
        console.error('❌ GEMINI_API_KEY não encontrada!');
        process.exit(1);
    }

    // Modelos para testar
    const models = [
        'nano-banana-pro-preview',
        'imagen-4.0-generate-preview-06-06',
        'imagen-4.0-ultra-generate-preview-06-06'
    ];

    const testPrompt = "A simple red circle on white background";

    for (const modelName of models) {
        console.log(`\n🧪 Testando modelo: ${modelName}`);
        console.log('-'.repeat(70));

        try {
            // Tentar endpoint :predict
            const predictUrl = `https://generativelanguage.googleapis.com/v1beta/models/${modelName}:predict?key=${apiKey}`;
            
            console.log(`📡 Endpoint: ${modelName}:predict`);
            
            const requestBody = {
                instances: [{
                    prompt: testPrompt
                }],
                parameters: {
                    sampleCount: 1,
                    aspectRatio: "1:1"
                }
            };

            const response = await fetch(predictUrl, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(requestBody)
            });

            const responseText = await response.text();
            
            if (!response.ok) {
                console.log(`❌ Erro (${response.status}):`);
                try {
                    const errorJson = JSON.parse(responseText);
                    console.log(JSON.stringify(errorJson, null, 2));
                } catch {
                    console.log(responseText.substring(0, 500));
                }
                continue;
            }

            const data = JSON.parse(responseText);
            console.log('✅ Resposta recebida!');
            console.log('📦 Estrutura da resposta:');
            console.log(JSON.stringify(Object.keys(data), null, 2));
            
            // Verificar estrutura
            if (data.predictions && data.predictions[0]) {
                const pred = data.predictions[0];
                if (pred.bytesBase64Encoded) {
                    console.log(`✅ Imagem encontrada (base64, ${pred.bytesBase64Encoded.length} chars)`);
                } else if (pred.image) {
                    console.log(`✅ Imagem encontrada em pred.image`);
                } else {
                    console.log('⚠️  Estrutura de predição diferente:');
                    console.log(JSON.stringify(pred, null, 2).substring(0, 300));
                }
            } else if (data.images) {
                console.log(`✅ Imagens encontradas em data.images`);
            } else {
                console.log('⚠️  Estrutura de resposta diferente:');
                console.log(JSON.stringify(data, null, 2).substring(0, 500));
            }

        } catch (error) {
            console.error(`❌ Erro ao testar ${modelName}:`, error.message);
        }
    }

    // Testar também o endpoint generateContent (se disponível)
    console.log('\n\n🧪 Testando endpoint generateContent (alternativo)');
    console.log('-'.repeat(70));
    
    try {
        const generateUrl = `https://generativelanguage.googleapis.com/v1beta/models/nano-banana-pro-preview:generateContent?key=${apiKey}`;
        
        const requestBody = {
            contents: [{
                parts: [{
                    text: `Generate an image: ${testPrompt}`
                }]
            }]
        };

        const response = await fetch(generateUrl, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(requestBody)
        });

        const responseText = await response.text();
        
        if (!response.ok) {
            console.log(`❌ Erro (${response.status}):`);
            console.log(responseText.substring(0, 500));
        } else {
            const data = JSON.parse(responseText);
            console.log('✅ Resposta recebida!');
            console.log('📦 Estrutura:');
            console.log(JSON.stringify(Object.keys(data), null, 2));
        }
    } catch (error) {
        console.error(`❌ Erro:`, error.message);
    }

    console.log('\n' + '='.repeat(70));
    console.log('✅ Teste concluído!');
    console.log('='.repeat(70));
}

testNanoBanana().catch(error => {
    console.error('\n❌ Erro fatal:', error);
    process.exit(1);
});



