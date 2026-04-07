# Security Policy

📌 **Quick links**: [README](README.md) · [User Guide](docs/GUIDE.md) · [Contributing](CONTRIBUTING.md) · [Changelog](CHANGELOG.md)

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| 1.0.x   | ✅ Active support  |

## Reporting a Vulnerability

If you discover a security vulnerability in Prismo Tauri, please report it responsibly:

1. **Do not** open a public issue.
2. Email your findings to the maintainers at the contact information listed in the repository.
3. Include:
   - A description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

We will acknowledge your report within 48 hours and aim to release a fix within 7 days for critical issues.

## Security Considerations

### API Key Storage

- API keys entered in Settings are stored in the application's local configuration file (`prismo.config.json`).
- The configuration file is stored in the OS-specific app data directory.
- API keys are **not** encrypted at rest. Do not share your configuration files.

### File System Access

- Prismo Tauri requests access to `$APPDATA`, `$DOCUMENT`, `$HOME`, and `$RESOURCE` directories.
- File operations are scoped to these directories via Tauri's capability system.
- The app does not access files outside the declared scope.

### Content Security Policy

- The app enforces a CSP that restricts script sources and style sources.
- External network requests are limited to what is needed for audit execution.

### Dependencies

- Frontend dependencies are audited with `npm audit` before each release.
- Rust dependencies are managed via `Cargo.lock` for reproducible builds.
