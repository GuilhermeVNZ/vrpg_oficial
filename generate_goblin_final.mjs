import fs from 'fs';
import path from 'path';

async function main() {
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

    console.log(`Using API Key: ${apiKey.substring(0, 5)}...`);

    // Load reference images
    const refImage01 = fs.readFileSync('assets-and-models/sprites/monsters/goblin01/idle_01.png');
    const refImage06 = fs.readFileSync('assets-and-models/sprites/monsters/goblin01/idle_06.png');

    const ref01Base64 = refImage01.toString('base64');
    const ref06Base64 = refImage06.toString('base64');

    const frames = [
        {
            filename: "idle_07.png",
            prompt: "Dungeons and Dragons goblin warrior character in dark fantasy anime illustration style, viewed from top-down angle. Frame 7 of idle animation: head tilts slightly right, shoulders rising, scimitar arm lifts 15 degrees, shield shifts outward. Alert warrior pose. Match the exact style, character design, and composition from the reference images. Same green goblin with rusty helmet, leather armor, scimitar and shield on circular stone base. Solid dark grey background, NO checkered pattern.",
            refImages: [ref01Base64, ref06Base64]
        },
        {
            filename: "idle_08.png",
            prompt: "Dungeons and Dragons goblin warrior character in dark fantasy anime illustration style, viewed from top-down angle. Frame 8 of idle animation: Peak breathing - chest expanded, head turned 20 degrees right, shoulders high, scimitar extended outward, shield pulled in. Most dynamic alert frame. Match the exact style, character design, and composition from the reference images. Same green goblin with rusty helmet, leather armor, scimitar and shield on circular stone base. Solid dark grey background, NO checkered pattern.",
            refImages: [ref01Base64, ref06Base64]
        },
        {
            filename: "idle_09.png",
            prompt: "Dungeons and Dragons goblin warrior character in dark fantasy anime illustration style, viewed from top-down angle. Frame 9 of idle animation: Transition to neutral - head rotating center, shoulders dropping, chest relaxing, arms lowering. Bridges to frame 1. Match the exact style, character design, and composition from the reference images. Same green goblin with rusty helmet, leather armor, scimitar and shield on circular stone base. Solid dark grey background, NO checkered pattern.",
            refImages: [ref01Base64, ref06Base64]
        }
    ];

    const outputDir = path.join('assets-and-models', 'sprites', 'monsters', 'goblin01');

    // Using Imagen 4.0 Ultra model (Nano Banana Pro - working version)
    // Note: imagen-4.0-generate-preview-06-06 returns empty response
    // imagen-4.0-ultra-generate-preview-06-06 works correctly
    const modelName = "imagen-4.0-ultra-generate-preview-06-06";

    for (const frame of frames) {
        console.log(`\nGenerating ${frame.filename}...`);

        try {
            // Try the generateImages endpoint format
            const requestBody = {
                prompt: frame.prompt,
                number_of_images: 1,
                aspect_ratio: "1:1",
                safety_filter_level: "block_only_high",
                person_generation: "allow_adult",
                reference_images: frame.refImages.map(img => ({ image: { bytes_base64_encoded: img } }))
            };

            // Use :predict endpoint (works with imagen-4.0-ultra)
            const response = await fetch(
                `https://generativelanguage.googleapis.com/v1beta/models/${modelName}:predict?key=${apiKey}`,
                {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        instances: [{
                            prompt: frame.prompt,
                            // Reference images for style matching
                            reference_images: frame.refImages.map(img => ({
                                image: { bytesBase64Encoded: img }
                            }))
                        }],
                        parameters: {
                            sampleCount: 1,
                            aspectRatio: "1:1"
                        }
                    })
                }
            );

            if (!response.ok) {
                const errorText = await response.text();
                console.error(`API Error (${response.status}):`, errorText.substring(0, 500));
                continue;
            }

            const data = await response.json();

            // Parse response from :predict endpoint
            let imageData = null;
            if (data.predictions && data.predictions[0]) {
                imageData = data.predictions[0].bytesBase64Encoded || data.predictions[0].image?.bytesBase64Encoded;
            } else if (data.generated_images && data.generated_images[0]) {
                imageData = data.generated_images[0].image?.bytes_base64_encoded || data.generated_images[0].bytesBase64Encoded;
            }

            if (imageData) {
                const buffer = Buffer.from(imageData, 'base64');
                const outputPath = path.join(outputDir, frame.filename);
                fs.writeFileSync(outputPath, buffer);
                console.log(`✓ Saved ${frame.filename} (${(buffer.length / 1024).toFixed(1)} KB)`);
            } else {
                console.error(`✗ No image data in response`);
                console.log('Response structure:', JSON.stringify(Object.keys(data), null, 2));
                console.log('Response sample:', JSON.stringify(data).substring(0, 500));
            }

        } catch (error) {
            console.error(`✗ Failed: ${error.message}`);
        }
    }

    console.log('\n✓ Complete!');
}

main();
