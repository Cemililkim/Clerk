# Scripts

This directory contains utility scripts for the Clerk project.

## fix-colors.py

### Purpose
Replaces hardcoded purple theme colors in CSS files with CSS variables, enabling dynamic theme color switching across the entire application.

### What it does
- Scans all CSS files in `src/components/`
- Replaces hardcoded hex colors (#9333ea, #a855f7, etc.) with CSS variables (var(--primary), var(--primary-light), etc.)
- Updates rgba() values for shadows and opacity
- Converts gradients to use CSS variables
- Fixes the loading spinner color in App.css

### Color Mappings
| Original Color | CSS Variable | Usage |
|---------------|-------------|-------|
| `#9333ea` | `var(--primary)` | Primary brand color |
| `#a855f7` | `var(--primary-light)` | Lighter variant |
| `#c084fc` | `var(--primary-lighter)` | Even lighter |
| `#7c3aed` | `var(--primary-dark)` | Darker variant |
| `#f3e8ff` | `var(--primary-bg-light)` | Light background |
| `#faf5ff` | `var(--primary-bg-lighter)` | Lighter background |
| `#e9d5ff` | `var(--primary-border)` | Border color |
| `linear-gradient(...)` | `var(--primary-gradient)` | Gradient |
| `rgba(147, 51, 234, 0.2)` | `var(--primary-shadow)` | Shadow |
| `rgba(147, 51, 234, 0.3)` | `var(--primary-shadow-hover)` | Hover shadow |

### Usage

```bash
# From project root
python scripts/fix-colors.py
```

### Output
```
🎨 Starting color replacement...

✅ src/components\AuditLog.css: 17 changes made
✅ src/components\EnvironmentModal.css: 7 changes made
✅ src/components\VariableList.css: 10 changes made
...

✨ Color replacement complete!
```

### When to use
- Adding new CSS files with hardcoded colors
- After importing third-party components
- Converting legacy styles to use the theme system
- Bulk updates when adding new theme colors

### Requirements
- Python 3.6+
- No external dependencies (uses only standard library)

### Notes
- The script is idempotent - running it multiple times won't cause issues
- It only replaces exact color matches (case-insensitive)
- Preserves file encoding (UTF-8)
- Creates backups recommended before running on new files

## Future Scripts

This directory is intended for additional utility scripts such as:
- Theme generator for new color schemes
- CSS minification/optimization
- Component scaffolding
- Build automation
- Testing utilities
