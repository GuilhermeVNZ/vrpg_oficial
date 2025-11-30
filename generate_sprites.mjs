import fs from 'fs';
import path from 'path';
// We don't strictly need the SDK if we use fetch directly, which avoids package issues.
// But let's try to import it just in case we want to use it later.
// actually, let's skip the SDK import to be 100% safe from package resolution issues 
// and just use the REST API with fetch, which is built-in in Node 22.

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
            // Check structure of response. It might be 'predictions' or something else depending on the exact endpoint version.
            // For Vertex AI it's predictions, for Gemini API it might be different.
            // But let's assume predictions for now as per documentation for Imagen on Vertex/Gemini.

            let b64 = null;
            if (data.predictions && data.predictions[0] && data.predictions[0].bytesBase64Encoded) {
                b64 = data.predictions[0].bytesBase64Encoded;
            } else if (data.images && data.images[0]) {
                // Some endpoints return { images: [ ... ] }
                b64 = data.images[0];
            }

            if (b64) {
                const buffer = Buffer.from(b64, 'base64');
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

main();
