# Contributing to Prismo Tauri

Thank you for your interest in contributing to Prismo Tauri! This guide will help you get started.

📌 **Quick links**: [README](README.md) · [User Guide](docs/GUIDE.md) · [Changelog](CHANGELOG.md) · [Security](SECURITY.md) · [Brand Guidelines](assets/branding/BRAND.md)

## Getting Started

### Prerequisites

- [Node.js 22+](https://nodejs.org)
- [Rust](https://rustup.rs) (latest stable)
- Platform-specific dependencies:
  - **Linux**: `libwebkit2gtk-4.1-dev librsvg2-dev libgtk-3-dev libsoup-3.0-dev`
  - **macOS**: Xcode Command Line Tools
  - **Windows**: WebView2 (pre-installed on Windows 10/11)

### Setup

```bash
git clone https://github.com/diShine-digital-agency/prismo-tauri.git
cd prismo-tauri
npm install
npm run tauri dev
```

## Development Workflow

1. **Create a branch** from `main` for your changes.
2. **Make your changes** following the code style guidelines below.
3. **Test** your changes by running `npm run build` (frontend) and `npm run tauri dev` (full app).
4. **Commit** with clear, descriptive messages.
5. **Open a Pull Request** against `main`.

## Code Style

### TypeScript / React

- Use strict TypeScript (`strict: true` in tsconfig.json).
- Define interfaces for all component props.
- Use functional components with hooks.
- Use Tailwind CSS utility classes for styling.
- Prefer named exports for pages, default exports for components.

### Rust

- Follow standard Rust formatting (`cargo fmt`).
- Use `Result<T, String>` for Tauri command error handling.
- Derive `Serialize`, `Deserialize`, `Clone` on shared structs.

### Commits

- Use clear, imperative commit messages: `Add feature X`, `Fix bug in Y`.
- Keep commits focused on a single change.

## Project Structure

| Directory | Purpose |
|-----------|---------|
| `src/` | React frontend (pages, components, styles) |
| `src-tauri/` | Rust backend (Tauri commands, config) |
| `toolkit/` | Audit prompt templates and export assets |
| `assets/` | Brand guidelines and color definitions |
| `docs/` | Examples and documentation |

## Adding a New Audit Prompt

1. Create a new `.md` file in `toolkit/prompts/<category>/`.
2. Add the audit definition in `src-tauri/src/lib.rs` (`get_audit_prompts`).
3. Add the matching entry in `src/pages/AuditRunner.tsx`.
4. Add an icon mapping in the `auditIcons` record.
5. Update the README audit count if adding a new category.

## Reporting Issues

- Use [GitHub Issues](https://github.com/diShine-digital-agency/prismo-tauri/issues).
- Include your OS, Node.js version, and Rust version.
- Provide steps to reproduce the issue.

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
