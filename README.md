# PackSwitcher

A macOS menubar app that automatically translates package manager commands between npm, pnpm, yarn, and bun.

## Features

- 🔄 **Automatic Command Translation** - Copy any package manager command and it gets instantly translated to your preferred package manager
- 📦 **Universal Support** - Works with npm, pnpm, yarn, and bun
- 🏃‍♂️ **Runner Commands** - Supports npx, pnpx, pnpm dlx, bunx, and yarn dlx
- ⚡ **Real-time Monitoring** - Monitors your clipboard automatically
- 🎯 **Smart Translation** - Handles global installs, script commands, and package manager specific syntax
- 🖥️ **Native macOS Design** - Clean, minimal menubar interface

## How It Works

1. **Select your preferred package manager** (npm, pnpm, yarn, or bun)
2. **Start monitoring** by clicking the toggle button
3. **Copy any package manager command** - it automatically gets translated and replaced in your clipboard

### Translation Examples

| Original Command | Preferred: pnpm | Preferred: yarn | Preferred: bun |
|------------------|----------------|----------------|----------------|
| `npm install react` | `pnpm add react` | `yarn add react` | `bun add react` |
| `npx create-react-app` | `pnpx create-react-app` | `yarn dlx create-react-app` | `bunx create-react-app` |
| `pnpx create-next-app` | `pnpx create-next-app` | `yarn dlx create-next-app` | `bunx create-next-app` |
| `pnpm dlx create-next-app` | `pnpm dlx create-next-app` | `yarn dlx create-next-app` | `bunx create-next-app` |
| `yarn build` | `pnpm run build` | `yarn build` | `bun run build` |
| `bun install -g typescript` | `pnpm add -g typescript` | `yarn global add typescript` | `bun add -g typescript` |
| `bunx prisma generate` | `pnpx prisma generate` | `yarn dlx prisma generate` | `npx prisma generate` |

## Installation

### Download

Download the latest release from the [Releases page](https://github.com/MartinPTielemans/packswitcher/releases).

### Build from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/MartinPTielemans/packswitcher.git
   cd packswitcher
   ```

2. Install dependencies:
   ```bash
   pnpm install
   ```

3. Build the app:
   ```bash
   pnpm tauri build
   ```

## Usage

1. Click the PackSwitcher icon in your menubar
2. Select your preferred package manager
3. Click "Start Monitoring"
4. Copy any package manager command - it will be automatically translated!

## Supported Commands

- **Package Management**: `install`, `add`, `uninstall`, `remove`
- **Script Running**: `run`, `start`, `build`, `test`, `dev`
- **Global Installs**: `-g`, `--global`
- **Package Runners**: `npx`, `pnpx`, `pnpm dlx`, `bunx`, `yarn dlx`

## Requirements

- macOS 10.15 or later
- Clipboard access permission

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
