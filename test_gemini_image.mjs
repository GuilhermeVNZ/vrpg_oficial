import fs from 'fs';
import path from 'path';

async function main() {
    // Read API Key from .env
    let apiKey = '';
    try {
        const envContent = fs.readFileSync('.env', 'utf8');
        const match = envContent.match(/GEMINI_API_KEY=(.*)/);
        if (match) {
            apiKey = match[1].trim();
        }
    } catch (e) {
        console.error('Error reading .env:', e);
        process.exit(1);
    }

    if (!apiKey) {
        console.error('GEMINI_API_KEY not found in .env');
        process.exit(1);
    }

    console.log(`Found API Key: ${apiKey.substring(0, 5)}...`);

    // Load reference images
    const refImage01 = fs.readFileSync('assets-and-models/sprites/monsters/goblin01/idle_01.png');
    const refImage06 = fs.readFileSync('assets-and-models/sprites/monsters/goblin01/idle_06.png');

    const refImage01Base64 = `data:image/png;base64,${refImage01.toString('base64')}`;
    const refImage06Base64 = `data:image/png;base64,${refImage06.toString('base64')}`;

    // Enhanced prompts with more natural movement descriptions
    const frames = [
        {
            filename: "idle_07.png",
            prompt: "Frame 7 of idle animation: Goblin's head tilts slightly right, shoulders rising, scimitar arm lifts 15 degrees, shield shifts outward. Alert warrior pose.",
            refImages: [refImage01Base64, refImage06Base64]
        },
        {
            filename: "idle_08.png",
            prompt: "Frame 8 of idle animation: Peak breathing - chest expanded, head turned 20 degrees right, shoulders high, scimitar extended outward, shield pulled in. Most dynamic alert frame.",
            refImages: [refImage01Base64, refImage06Base64]
        },
        {
            filename: "idle_09.png",
            prompt: "Frame 9 of idle animation: Transition to neutral - head rotating center, shoulders dropping, chest relaxing, arms lowering. Bridges to frame 1.",
            refImages: [refImage01Base64, refImage06Base64]
        }
    ];

    const outputDir = path.join('assets-and-models', 'sprites', 'monsters', 'goblin01');

    // Try generateContent endpoint with multimodal input
    for (const frame of frames) {
        console.log(`\nGenerating ${frame.filename}...`);

        try {
            const requestBody = {
                contents: [{
                    parts: [
                        { text: `Generate this exact image: Dungeons and Dragons goblin warrior, dark fantasy anime style, top-down view, circular stone base. ${frame.prompt} Match the style and character from these reference images exactly. Same green goblin with rusty helmet, leather armor, scimitar and shield. Solid dark grey background, NO checkered pattern. 1:1 aspect ratio.` },
                        { inline_data: { mime_type: "image/png", data: frame.refImages[0].split(',')[1] } },
                        { inline_data: { mime_type: "image/png", data: frame.refImages[1].split(',')[1] } }
                    ]
                }],
                generationConfig: {
                    temperature: 0.4,
                    topK: 32,
                    topP: 1,
                    maxOutputTokens: 4096,
                }
            };

            const response = await fetch(
                `https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key=${apiKey}`,
                {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(requestBody)
                }
            );

            if (!response.ok) {
                const errorText = await response.text();
                console.error(`API Error (${response.status}):`, errorText.substring(0, 500));
                continue;
            }

            const data = await response.json();
            console.log('Response:', JSON.stringify(data).substring(0, 300));

        } catch (error) {
            console.error(`✗ Failed to generate ${frame.filename}:`, error.message);
        }
    }

    console.log('\n✓ Script complete!');
}

main();
