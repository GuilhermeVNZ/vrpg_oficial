#!/usr/bin/env python3
"""
Generate detailed, customized animation prompts for all D&D 5e monsters.
Each monster gets 9 unique, detailed prompts following ART_DIRECTION_SPRITES.md
"""

import os
from pathlib import Path

# Monster database with detailed descriptions
MONSTERS = {
    "aboleth": {
        "desc": "massive eel-like aberration with three eyes, four tentacles, mucus-covered body",
        "animation": ["floating still", "tentacles begin to writhe", "eyes glowing brighter", "tentacles fully extended", "psychic energy radiating", "tentacles retracting", "eyes dimming", "settling", "return to rest"]
    },
    "adult_black_dragon": {
        "desc": "huge black-scaled dragon with horns curving forward, acid dripping from jaws, tattered wings",
        "animation": ["perched menacingly", "wings unfurling", "head rising", "jaws opening wide", "roaring with acid spray", "wings spread maximum", "jaws closing", "wings folding", "return to perch"]
    },
    # Add all 348 monsters here...
}

def generate_prompt(monster_name, frame_num, monster_data):
    """Generate a detailed prompt for a specific frame"""
    desc = monster_data["desc"]
    action = monster_data["animation"][frame_num - 1]
    
    prompt = f"""Dungeons and Dragons dark fantasy anime illustration, {monster_name.replace('_', ' ')} character, {desc}. Frame {frame_num} of 9-frame idle animation: {action}. Viewed from top-down angle (80-90 degrees), standing on circular base, 2D painted illustration style with high-contrast dramatic lighting, detailed but readable from above, strong silhouette, 1:1 square aspect ratio, fits entirely within frame, solid dark grey background NO CHECKERED PATTERN, dark fantasy anime art style similar to Solo Leveling."""
    
    return prompt

# This is a template - full implementation would process all 350 monsters
print("Generating detailed prompts for all monsters...")
print("This will take a few minutes...")
