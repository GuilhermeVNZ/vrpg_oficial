import fs from 'fs';

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

    console.log(`Using Key: ${apiKey.substring(0, 5)}...`);

    try {
        const response = await fetch(`https://generativelanguage.googleapis.com/v1beta/models?key=${apiKey}`);
        const data = await response.json();

        if (data.models) {
            console.log("Available Imagen Models:");
            data.models.forEach(m => {
                if (m.name.includes('imagen')) {
                    console.log(`- ${m.name}`);
                }
            });
        } else {
            console.log("No models found or error:", JSON.stringify(data, null, 2));
        }

    } catch (e) {
        console.error("Error listing models:", e);
    }
}

main();
