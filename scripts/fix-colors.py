"""
CSS Color Replacement Script

This script replaces hardcoded purple theme colors in CSS files with CSS variables,
enabling dynamic theme color switching across the entire application.

Usage:
    python fix-colors.py

The script will:
1. Scan all CSS files in src/components/
2. Replace hardcoded hex colors with CSS variables
3. Replace hardcoded rgba values with CSS variables
4. Update gradients to use CSS variables
"""

import os
import re

# Color replacement mapping (order matters - specific before general)
color_replacements = {
    # Gradient
    r'linear-gradient\(135deg,\s*#a855f7\s+0%,\s*#9333ea\s+100%\)': 'var(--primary-gradient)',
    # Hex colors
    r'#9333ea\b': 'var(--primary)',
    r'#a855f7\b': 'var(--primary-light)',
    r'#c084fc\b': 'var(--primary-lighter)',
    r'#7c3aed\b': 'var(--primary-dark)',
    r'#f3e8ff\b': 'var(--primary-bg-light)',
    r'#faf5ff\b': 'var(--primary-bg-lighter)',
    r'#e9d5ff\b': 'var(--primary-border)',
    # rgba colors with color-mix (modern CSS)
    r'rgba\(\s*243,\s*232,\s*255,\s*0\.4\)': 'color-mix(in srgb, var(--primary-bg-light) 40%, transparent)',
    r'rgba\(\s*233,\s*213,\s*255,\s*0\.6\)': 'color-mix(in srgb, var(--primary-border) 60%, transparent)',
    r'rgba\(\s*168,\s*85,\s*247,\s*0\.05\)': 'color-mix(in srgb, var(--primary-light) 5%, transparent)',
    r'rgba\(\s*168,\s*85,\s*247,\s*0\.1\)': 'color-mix(in srgb, var(--primary) 10%, transparent)',
    r'rgba\(\s*168,\s*85,\s*247,\s*0\.15\)': 'color-mix(in srgb, var(--primary-light) 15%, transparent)',
    r'rgba\(\s*168,\s*85,\s*247,\s*0\.2\)': 'color-mix(in srgb, var(--primary-light) 20%, transparent)',
    r'rgba\(\s*168,\s*85,\s*247,\s*0\.3\)': 'color-mix(in srgb, var(--primary) 30%, transparent)',
    r'rgba\(\s*147,\s*51,\s*234,\s*0\.02\)': 'color-mix(in srgb, var(--primary) 2%, transparent)',
    r'rgba\(\s*147,\s*51,\s*234,\s*0\.05\)': 'color-mix(in srgb, var(--primary) 5%, transparent)',
    r'rgba\(\s*147,\s*51,\s*234,\s*0\.1\)': 'color-mix(in srgb, var(--primary) 10%, transparent)',
    r'rgba\(\s*147,\s*51,\s*234,\s*0\.15\)': 'color-mix(in srgb, var(--primary) 15%, transparent)',
    # Standard shadows (keep as variables)
    r'rgba\(\s*147,\s*51,\s*234,\s*0\.2\)': 'var(--primary-shadow)',
    r'rgba\(\s*147,\s*51,\s*234,\s*0\.3\)': 'var(--primary-shadow-hover)',
}

# File patterns to process
patterns = [
    'src/components/*.css',
    'src/styles/dark-mode.css'
]

def fix_colors_in_file(filepath):
    """
    Replace all hardcoded purple colors in a file with CSS variables.
    
    Args:
        filepath: Path to the CSS file to process
        
    Returns:
        bool: True if changes were made, False otherwise
    """
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        changes_made = 0
        
        # Apply each color replacement
        for pattern, replacement in color_replacements.items():
            new_content = re.sub(pattern, replacement, content)
            if new_content != content:
                changes_count = len(re.findall(pattern, content))
                changes_made += changes_count
                content = new_content
        
        # Save file if changes were made
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✅ {filepath}: {changes_made} changes made")
            return True
        else:
            print(f"⏭️  {filepath}: No changes needed")
            return False
            
    except Exception as e:
        print(f"❌ {filepath}: Error - {e}")
        return False

def main():
    """Main function to process all CSS files."""
    print("🎨 Starting color replacement...\n")
    
    # Process component CSS files
    component_dir = 'src/components'
    if os.path.exists(component_dir):
        for filename in os.listdir(component_dir):
            if filename.endswith('.css'):
                filepath = os.path.join(component_dir, filename)
                fix_colors_in_file(filepath)
    
    # Process dark-mode.css
    dark_mode_file = 'src/styles/dark-mode.css'
    if os.path.exists(dark_mode_file):
        fix_colors_in_file(dark_mode_file)
    
    # Process App.css (special case for loading spinner)
    app_css_file = 'src/styles/App.css'
    if os.path.exists(app_css_file):
        with open(app_css_file, 'r', encoding='utf-8') as f:
            content = f.read()
        # Fix loading spinner color
        content = re.sub(r'\.app-loading-spinner \{[^}]+color:\s*#9333ea;', 
                        '.app-loading-spinner {\n  color: var(--primary);', content)
        with open(app_css_file, 'w', encoding='utf-8') as f:
            f.write(content)
        print(f"✅ {app_css_file}: Loading spinner fixed")
    
    print("\n✨ Color replacement complete!")

if __name__ == '__main__':
    main()
