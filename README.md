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

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) (LTS version)
- [pnpm](https://pnpm.io/) package manager
- [Rust](https://rustup.rs/) toolchain

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/MartinPTielemans/PackSwitcher.git
   cd PackSwitcher
   ```

2. Install dependencies:
   ```bash
   pnpm install
   ```

3. Start the development server:
   ```bash
   pnpm tauri dev
   ```

### Code Quality & Formatting

This project uses automated code formatting and linting to maintain consistent code quality.

#### Automatic Formatting

The project is configured with:
- **Prettier** for code formatting
- **ESLint** for code linting
- **TypeScript** strict mode for type safety
- **Pre-commit hooks** to automatically format and lint staged files

#### Available Scripts

| Command | Description |
|---------|-------------|
| `pnpm run format` | Format all files with Prettier |
| `pnpm run format:check` | Check if all files are properly formatted |
| `pnpm run lint` | Run ESLint to check for issues |
| `pnpm run lint:fix` | Run ESLint and fix auto-fixable issues |
| `pnpm run type-check` | Run TypeScript type checking |
| `pnpm run check` | Run all checks (formatting, linting, type checking) |
| `pnpm run fix` | Fix all auto-fixable issues (formatting + linting) |
| `pnpm run ci` | Run full CI pipeline locally |

#### Pre-commit Hooks

The project uses [Husky](https://typicode.github.io/husky/) and [lint-staged](https://github.com/okonet/lint-staged) to automatically format and lint your code before commits:

- **Prettier** formats TypeScript, JavaScript, JSON, and Markdown files
- **ESLint** fixes auto-fixable linting issues
- Only staged files are processed for faster commits

#### IDE Setup

For the best development experience, we recommend using VS Code with the following extensions (automatically suggested when you open the project):

- **Prettier** - Code formatter
- **ESLint** - JavaScript/TypeScript linter
- **Tauri** - Tauri framework support
- **Rust Analyzer** - Rust language support
- **EditorConfig** - Consistent editor settings

The project includes VS Code settings that automatically:
- Format code on save
- Fix ESLint issues on save
- Organize imports on save

#### CI/CD

The GitHub Actions workflow automatically checks:
- ✅ Code formatting (Prettier)
- ✅ Code linting (ESLint)
- ✅ Type checking (TypeScript)
- ✅ Build success
- ✅ Rust code formatting and linting

If any checks fail, the CI will provide helpful error messages with suggestions on how to fix the issues.

### Building

Build the application for production:

```bash
pnpm tauri build
```

This will create platform-specific installers in the `src-tauri/target/release/bundle/` directory.

## License

MIT License - see [LICENSE](LICENSE.md) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
