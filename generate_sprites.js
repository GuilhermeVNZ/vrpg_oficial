const fs = require('fs');
const path = require('path');
const { GoogleGenerativeAI } = require('@google/generative-ai');

async function main() {
    // Read API Key from .env
    let apiKey = '';
    try {
        const envContent = fs.readFileSync('.env', 'utf8');
        const match = envContent.match(/NANOBANANA_API_KEY=(.*)/);
        if (match) {
            apiKey = match[1].trim();
        }
    } catch (e) {
        console.error('Error reading .env:', e);
        process.exit(1);
    }

    if (!apiKey) {
        console.error('NANOBANANA_API_KEY not found in .env');
        process.exit(1);
    }

    console.log(`Found API Key: ${apiKey.substring(0, 5)}...`);

    const genAI = new GoogleGenerativeAI(apiKey);

    // Note: The model name for image generation via Gemini API
    // Trying 'imagen-3.0-generate-001'
    const model = genAI.getGenerativeModel({ model: 'imagen-3.0-generate-001' });

    const prompts = {
        "idle_07.png": "Dungeons and Dragons goblin warrior character, dark fantasy anime illustration style, top-down view. Frame 7 of idle animation loop. Continuing from frame 6. Broader movement starts: Head begins to lift/turn, arms move outward away from body. More dynamic than previous frames. Tabletop miniature aesthetic on circular stone base. Solid dark grey background.",
        "idle_08.png": "Dungeons and Dragons goblin warrior character, dark fantasy anime illustration style, top-down view. Frame 8 of idle animation loop. Broader movement continues: Head turned slightly, arms extended further, weapon brandished slightly. Exaggerated breathing/movement. Tabletop miniature aesthetic on circular stone base. Solid dark grey background.",
        "idle_09.png": "Dungeons and Dragons goblin warrior character, dark fantasy anime illustration style, top-down view. Frame 9 of idle animation loop. Peak of broad movement: Shoulders high, chest fully expanded, head active. Ready to loop back to neutral. Tabletop miniature aesthetic on circular stone base. Solid dark grey background."
    };

    const outputDir = path.join('assets-and-models', 'sprites', 'monsters', 'goblin01');
    if (!fs.existsSync(outputDir)) {
        fs.mkdirSync(outputDir, { recursive: true });
    }

    for (const [filename, prompt] of Object.entries(prompts)) {
        console.log(`Generating ${filename}...`);
        try {
            // The JS SDK might not have a dedicated generateImage method yet on the generic model object
            // We might need to call the API directly if the SDK doesn't support it.
            // But let's try the SDK first. If it fails, we catch and try fetch.

            // Hypothetical SDK usage for image generation (it varies by version)
            // If this fails, we will fall back to REST API in the catch block.

            // Actually, let's just use REST API directly to be safe and avoid SDK version issues.
            throw new Error("SDK image generation not guaranteed, switching to REST");

        } catch (e) {
            // Fallback to REST API
            try {
                const response = await fetch(`https://generativelanguage.googleapis.com/v1beta/models/imagen-3.0-generate-001:predict?key=${apiKey}`, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({
                        instances: [
                            { prompt: prompt }
                        ],
                        parameters: {
                            sampleCount: 1,
                            aspectRatio: "1:1"
                        }
                    })
                });

                if (!response.ok) {
                    const errorText = await response.text();
                    throw new Error(`API Error: ${response.status} ${errorText}`);
                }

                const data = await response.json();
                if (data.predictions && data.predictions[0] && data.predictions[0].bytesBase64Encoded) {
                    const buffer = Buffer.from(data.predictions[0].bytesBase64Encoded, 'base64');
                    fs.writeFileSync(path.join(outputDir, filename), buffer);
                    console.log(`Saved ${filename}`);
                } else {
                    console.error(`No image data in response for ${filename}`, JSON.stringify(data).substring(0, 200));
                }

            } catch (restError) {
                console.error(`Failed to generate ${filename}:`, restError.message);
            }
        }
    }
}

main();
