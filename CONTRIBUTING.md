# Contributing to PackSwitcher

Thank you for your interest in contributing to PackSwitcher! This guide will help you get started with development and ensure your contributions meet our quality standards.

## 🚀 Quick Start

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/your-username/PackSwitcher.git
   cd PackSwitcher
   ```

2. **Install dependencies**
   ```bash
   pnpm install
   ```

3. **Start developing**
   ```bash
   pnpm tauri dev
   ```

## 📋 Development Workflow

### Before You Start

1. **Check existing issues** - Look for existing issues or create a new one to discuss your changes
2. **Create a branch** - Use a descriptive name: `feature/your-feature-name` or `fix/issue-description`

### Development Process

1. **Make your changes** following our [code style guidelines](#code-style)
2. **Test your changes** - Ensure everything works as expected
3. **Run quality checks** - Use our automated tools to verify code quality
4. **Commit your changes** - Our pre-commit hooks will automatically format and lint your code
5. **Push and create a PR** - Provide a clear description of your changes

## 🛠️ Code Quality Tools

We use automated tools to maintain consistent code quality:

### Available Scripts

| Command | Purpose | When to Use |
|---------|---------|-------------|
| `pnpm run check` | Run all checks (format, lint, type) | Before committing |
| `pnpm run fix` | Fix all auto-fixable issues | When you have formatting/linting errors |
| `pnpm run format` | Format code with Prettier | Rarely needed (auto-runs on commit) |
| `pnpm run lint` | Check for linting issues | To see what needs fixing |
| `pnpm run type-check` | TypeScript type checking | To catch type errors |
| `pnpm run ci` | Full CI pipeline locally | Before pushing to GitHub |

### Pre-commit Hooks

We use Husky + lint-staged to automatically:
- ✅ Format code with Prettier
- ✅ Fix ESLint issues
- ✅ Only process staged files (fast commits)

**No need to run formatting manually** - it happens automatically when you commit!

### IDE Setup (Recommended)

For the best experience, use **VS Code** with our recommended extensions:
- Prettier (code formatting)
- ESLint (linting)
- Tauri (framework support)
- Rust Analyzer (Rust support)

The project includes VS Code settings that automatically format and fix issues on save.

## 📝 Code Style Guidelines

### TypeScript/React

- **Use TypeScript strict mode** - All functions should have explicit return types
- **Prefer functional components** - Use React hooks over class components
- **Use explicit types** - Avoid `any`, define proper interfaces
- **Error handling** - Always handle promise rejections and errors
- **Imports** - Use relative imports, organize imports automatically

### Example Code Style

```typescript
// ✅ Good - Explicit return type, proper error handling
const handleUpdate = async (): Promise<void> => {
  try {
    await invoke('update_command')
  } catch (error) {
    console.error('Failed to update:', error)
  }
}

// ✅ Good - Proper TypeScript interface
interface UpdateInfo {
  version: string
  releaseDate: string
}

// ❌ Bad - No return type, no error handling
const handleUpdate = async () => {
  await invoke('update_command')
}
```

### Rust

- Follow standard Rust formatting (runs automatically)
- Use `cargo clippy` recommendations
- Add documentation for public functions

## 🧪 Testing

### Frontend Testing

```bash
# Run all quality checks
pnpm run check

# Test build
pnpm run build

# Run full CI pipeline locally
pnpm run ci
```

### Tauri Testing

```bash
# Test in development
pnpm tauri dev

# Test production build
pnpm tauri build
```

## 📤 Pull Request Guidelines

### Before Submitting

1. **Run `pnpm run ci`** - Ensure all checks pass locally
2. **Test your changes** - Make sure everything works as expected
3. **Update documentation** - If you changed functionality, update README/docs
4. **Write clear commit messages** - Use conventional commit format when possible

### PR Description Template

```markdown
## 🎯 What does this PR do?

Brief description of your changes

## 🔧 Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## ✅ Testing

- [ ] Tested locally with `pnpm run ci`
- [ ] Tested in development mode
- [ ] Tested production build (if applicable)

## 📷 Screenshots (if applicable)

Add screenshots for UI changes
```

### CI Checks

Your PR must pass all automated checks:
- ✅ Code formatting (Prettier)
- ✅ Linting (ESLint)
- ✅ Type checking (TypeScript)
- ✅ Build success
- ✅ Rust formatting and linting

## 🐛 Reporting Issues

When reporting bugs, please include:

1. **Operating System** and version
2. **Steps to reproduce** the issue
3. **Expected behavior** vs actual behavior
4. **Screenshots or logs** if applicable
5. **Environment details** (package manager versions, etc.)

## 🎉 Recognition

All contributors will be recognized in our release notes and GitHub contributors list!

## 📞 Need Help?

- **GitHub Issues** - For bugs and feature requests
- **GitHub Discussions** - For questions and general discussion

Thank you for contributing to PackSwitcher! 🚀
