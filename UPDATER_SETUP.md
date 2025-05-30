# Auto-Updater Setup Guide

This guide will help you set up the auto-updater functionality for your PackSwitcher application.

## Step 1: Generate Signing Keys

First, you need to generate a public/private key pair for signing your updates:

```bash
# Create the .tauri directory if it doesn't exist
mkdir -p ~/.tauri

# Generate the signing keys (you'll be prompted for a password)
pnpm tauri signer generate -w ~/.tauri/packswitcher.key

# Alternative if pnpm doesn't work:
npx @tauri-apps/cli@latest signer generate -w ~/.tauri/packswitcher.key
```

This will generate:
- A private key file at `~/.tauri/packswitcher.key` (keep this secret!)
- A public key that will be displayed in the terminal

## Step 2: Update Configuration

1. Copy the public key from the terminal output
2. Update `src-tauri/tauri.conf.json` and replace `YOUR_PUBLIC_KEY_WILL_GO_HERE` with your actual public key
3. Update the GitHub repository URL in the endpoints array to match your actual repository

## Step 3: Set Up GitHub Secrets

Go to your GitHub repository settings and add these secrets:

1. `TAURI_PRIVATE_KEY`: The content of your private key file
2. `TAURI_KEY_PASSWORD`: The password you used when generating the key

## Step 4: Enable GitHub Actions Permissions

1. Go to your repository Settings → Actions → General
2. Under "Workflow permissions", select "Read and write permissions"
3. Click "Save"

## Step 5: Create a Release

To trigger the auto-updater workflow:

```bash
git tag v1.0.1
git push origin v1.0.1
```

The GitHub Action will build your app and create the necessary update files. 