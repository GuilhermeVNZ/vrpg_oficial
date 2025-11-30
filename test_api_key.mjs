#!/usr/bin/env node
/**
 * Script de teste para verificar se a API key do Google AI Studio está configurada corretamente
 * 
 * Uso: node test_api_key.mjs
 */

import fs from 'fs';
import path from 'path';

async function testApiKey() {
    console.log('='.repeat(70));
    console.log('Teste de Configuração da API Key do Google AI Studio');
    console.log('='.repeat(70));
    console.log();

    // 1. Verificar arquivo .env
    console.log('[1/4] Verificando arquivo .env...');
    const envPath = path.join(process.cwd(), '.env');
    
    if (!fs.existsSync(envPath)) {
        console.error('❌ Arquivo .env não encontrado!');
        console.log('\n📝 Crie um arquivo .env na raiz do projeto com:');
        console.log('   GEMINI_API_KEY=sua-chave-aqui');
        console.log('\n💡 Você pode copiar o arquivo env.example como base:');
        console.log('   copy env.example .env');
        process.exit(1);
    }
    console.log('✅ Arquivo .env encontrado');

    // 2. Ler API Key
    console.log('\n[2/4] Lendo GEMINI_API_KEY do .env...');
    let apiKey = '';
    try {
        const envContent = fs.readFileSync(envPath, 'utf8');
        const match = envContent.match(/GEMINI_API_KEY=(.*)/);
        if (match) {
            apiKey = match[1].trim();
            // Remove aspas se houver
            apiKey = apiKey.replace(/^["']|["']$/g, '');
        }
    } catch (e) {
        console.error('❌ Erro ao ler .env:', e.message);
        process.exit(1);
    }

    if (!apiKey) {
        console.error('❌ GEMINI_API_KEY não encontrada no .env!');
        console.log('\n📝 Adicione a seguinte linha no arquivo .env:');
        console.log('   GEMINI_API_KEY=sua-chave-aqui');
        process.exit(1);
    }

    if (apiKey.length < 20) {
        console.warn('⚠️  A API key parece muito curta. Verifique se está completa.');
    }

    console.log(`✅ API Key encontrada: ${apiKey.substring(0, 10)}...${apiKey.substring(apiKey.length - 4)}`);
    console.log(`   Tamanho: ${apiKey.length} caracteres`);

    // 3. Testar conexão com API (listar modelos)
    console.log('\n[3/4] Testando conexão com a API...');
    try {
        // Teste simples: tentar listar modelos disponíveis
        // Usando um endpoint que não requer modelo específico
        const testUrl = `https://generativelanguage.googleapis.com/v1beta/models?key=${apiKey}`;
        
        const response = await fetch(testUrl, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json'
            }
        });

        if (!response.ok) {
            const errorText = await response.text();
            let errorJson;
            try {
                errorJson = JSON.parse(errorText);
            } catch {
                errorJson = { error: { message: errorText } };
            }

            console.error(`❌ Erro na API (${response.status}):`);
            console.error(`   ${errorJson.error?.message || errorText.substring(0, 200)}`);

            if (response.status === 401) {
                console.log('\n💡 Possíveis causas:');
                console.log('   - API key inválida ou expirada');
                console.log('   - API key não tem permissões necessárias');
                console.log('   - Gere uma nova chave no Google AI Studio');
            } else if (response.status === 403) {
                console.log('\n💡 Possíveis causas:');
                console.log('   - Faturamento não ativado (necessário para Nano Banana Pro)');
                console.log('   - API não habilitada no Google Cloud Console');
                console.log('   - Verifique as permissões da API key');
            } else if (response.status === 429) {
                console.log('\n💡 Possíveis causas:');
                console.log('   - Limite de requisições excedido');
                console.log('   - Aguarde alguns minutos e tente novamente');
            }

            process.exit(1);
        }

        const data = await response.json();
        const models = data.models || [];
        
        console.log(`✅ Conexão com API bem-sucedida!`);
        console.log(`   Encontrados ${models.length} modelos disponíveis`);

        // Filtrar modelos relevantes
        const imagenModels = models.filter(m => m.name?.includes('imagen'));
        const geminiModels = models.filter(m => m.name?.includes('gemini'));

        if (imagenModels.length > 0) {
            console.log(`\n   📸 Modelos Imagen (Geração de Imagens):`);
            imagenModels.slice(0, 5).forEach(m => {
                console.log(`      - ${m.name?.replace('models/', '') || m.name}`);
            });
        }

        if (geminiModels.length > 0) {
            console.log(`\n   🤖 Modelos Gemini (LLM):`);
            geminiModels.slice(0, 5).forEach(m => {
                console.log(`      - ${m.name?.replace('models/', '') || m.name}`);
            });
        }

        // Verificar especificamente o Nano Banana Pro
        const nanoBanana = models.find(m => 
            m.name?.includes('imagen-4.0') || 
            m.name?.includes('nano-banana') ||
            m.name?.includes('preview-06-06')
        );

        if (nanoBanana) {
            console.log(`\n   🍌 Nano Banana Pro encontrado!`);
            console.log(`      Modelo: ${nanoBanana.name?.replace('models/', '') || nanoBanana.name}`);
        } else {
            console.log(`\n   ⚠️  Nano Banana Pro não encontrado na lista`);
            console.log(`      Isso pode ser normal se o modelo estiver em preview`);
            console.log(`      Tente usar diretamente: imagen-4.0-generate-preview-06-06`);
        }

    } catch (error) {
        console.error('❌ Erro ao conectar com a API:');
        console.error(`   ${error.message}`);
        
        if (error.message.includes('fetch')) {
            console.log('\n💡 Possíveis causas:');
            console.log('   - Problema de conexão com a internet');
            console.log('   - Firewall bloqueando requisições');
            console.log('   - Node.js muito antigo (requer Node 18+)');
        }
        
        process.exit(1);
    }

    // 4. Resumo final
    console.log('\n[4/4] Resumo da Configuração');
    console.log('='.repeat(70));
    console.log('✅ Configuração completa e funcionando!');
    console.log('\n📝 Próximos passos:');
    console.log('   1. Você pode usar os scripts de geração:');
    console.log('      - node generate_goblin_sprites.mjs');
    console.log('      - node test_gemini_image.mjs');
    console.log('   2. Verifique a documentação:');
    console.log('      - docs/GOOGLE_AI_STUDIO_SETUP.md');
    console.log('\n💡 Dica: Monitore o uso no Google Cloud Console para evitar custos inesperados');
    console.log('='.repeat(70));
}

// Executar teste
testApiKey().catch(error => {
    console.error('\n❌ Erro fatal:', error);
    process.exit(1);
});



