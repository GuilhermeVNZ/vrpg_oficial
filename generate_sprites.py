import os
import google.generativeai as genai
from pathlib import Path

# Read API Key from .env manually since python-dotenv might not be installed
api_key = None
try:
    with open('.env', 'r') as f:
        for line in f:
            if line.startswith('NANOBANANA_API_KEY='):
                api_key = line.strip().split('=')[1]
                break
except Exception as e:
    print(f"Error reading .env: {e}")
    exit(1)

if not api_key:
    print("NANOBANANA_API_KEY not found in .env")
    exit(1)

print(f"Found API Key: {api_key[:5]}...")

# Configure Gemini
genai.configure(api_key=api_key)

# Prompts for the remaining frames
prompts = {
    "idle_07.png": "Dungeons and Dragons goblin warrior character, dark fantasy anime illustration style, top-down view. Frame 7 of idle animation loop. Continuing from frame 6. Broader movement starts: Head begins to lift/turn, arms move outward away from body. More dynamic than previous frames. Tabletop miniature aesthetic on circular stone base. Solid dark grey background.",
    "idle_08.png": "Dungeons and Dragons goblin warrior character, dark fantasy anime illustration style, top-down view. Frame 8 of idle animation loop. Broader movement continues: Head turned slightly, arms extended further, weapon brandished slightly. Exaggerated breathing/movement. Tabletop miniature aesthetic on circular stone base. Solid dark grey background.",
    "idle_09.png": "Dungeons and Dragons goblin warrior character, dark fantasy anime illustration style, top-down view. Frame 9 of idle animation loop. Peak of broad movement: Shoulders high, chest fully expanded, head active. Ready to loop back to neutral. Tabletop miniature aesthetic on circular stone base. Solid dark grey background."
}

output_dir = Path("assets-and-models/sprites/monsters/goblin01")
output_dir.mkdir(parents=True, exist_ok=True)

# Generate images
# Note: Using imagen-3.0-generate-001 if available, or trying a fallback
model_name = "imagen-3.0-generate-001" 

print(f"Using model: {model_name}")

for filename, prompt in prompts.items():
    print(f"Generating {filename}...")
    try:
        # Attempt to use the image generation model
        # Note: The SDK syntax might vary slightly depending on version, trying standard approach
        imagen_model = genai.ImageGenerationModel(model_name)
        result = imagen_model.generate_images(
            prompt=prompt,
            number_of_images=1,
            aspect_ratio="1:1",
            safety_filter_level="block_only_high",
            person_generation="allow_adult",
        )
        
        if result and result.images:
            image = result.images[0]
            output_path = output_dir / filename
            image.save(location=str(output_path))
            print(f"Saved {output_path}")
        else:
            print(f"No images returned for {filename}")
            
    except Exception as e:
        print(f"Failed to generate {filename}: {e}")
        # Fallback: Try to print if it's a model not found error
        if "404" in str(e) or "not found" in str(e).lower():
             print("Model not found. You might not have access to Imagen 3 via this API key.")

print("Done.")
