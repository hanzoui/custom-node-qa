# Changelog

## Fixes Based on Feedback

### Issue 1: "Should list all valid project names"
**Fixed:** Setup now shows existing projects before asking for name.

```
Existing projects:
  - vue-node-cloud
  - mobile-testing

Give your project a name...
```

### Issue 2: "Failed with 'No such file or directory'"
**Fixed:**
- Checks if Desktop exists before writing
- Falls back to scripts/ folder if Desktop not found
- Better error handling for all file operations

### Issue 3: "Why save to desktop? Can't you just print?"
**Fixed:** Now asks user preference:

```
Save script to a file, or print here to copy? [y/N]
```

Default is 'N' (print to console) - user just copies from terminal.
If they choose 'y', they pick Desktop or scripts/ folder.

### Issue 4: "Too many files being created"
**Fixed:** Only creates files if user explicitly chooses to save them.
Default behavior prints everything to console for easy copying.

## Current User Flow

1. **Start setup:** `comfy-qa setup`
2. **See existing projects** (if any)
3. **Enter project name** (validated)
4. **Choose script delivery:**
   - Print to console (default) → copy from there
   - Save to file → choose location
5. **Follow instructions** for browser
6. **Export data** in browser
7. **Tool auto-detects** the exported file
8. **Done!**

## Testing

All changes tested and working:
- Shows existing projects correctly
- Handles missing Desktop gracefully
- Print-to-console works on all platforms
- File saving works when requested
- No crashes or errors
