# Version Synchronization

This directory contains scripts to keep version numbers synchronized across all project files.

## How it works

The `package.json` file serves as the single source of truth for the application version. The `sync-version.js` script automatically updates:

- `src-tauri/Cargo.toml` 
- `src-tauri/tauri.conf.json`

## Usage

### Manual sync
```bash
pnpm run sync-version
```

### Update version (recommended)
```bash
# This will update package.json and automatically sync other files
npm version patch   # 1.1.2 -> 1.1.3
npm version minor   # 1.1.2 -> 1.2.0  
npm version major   # 1.1.2 -> 2.0.0
```

### Automatic sync
The version sync runs automatically:
- Before each build (`pnpm run build`)
- After version updates (`npm version`)

## Files managed
- ✅ `package.json` (source of truth)
- ✅ `src-tauri/Cargo.toml`
- ✅ `src-tauri/tauri.conf.json` 