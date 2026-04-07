# Changelog

All notable changes to Prismo Tauri will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

📌 **Quick links**: [README](README.md) · [User Guide](docs/GUIDE.md) · [Contributing](CONTRIBUTING.md) · [Security](SECURITY.md)

## [1.0.0] — 2026-04-07

### Added

- **Desktop Application**: Cross-platform Tauri v2 app for Windows, macOS, and Linux
- **19 Audit Categories** across 9 domains:
  - System Health: Windows, Linux, macOS diagnosis, Log analysis, Network diagnostics
  - Web & Performance: Website performance, Tech stack analysis, Accessibility
  - SEO: Technical SEO, On-page SEO, Competitive analysis
  - MarTech & Data: MarTech stack, Data quality
  - Security: Website security, System security
  - Email & DNS: SPF, DKIM, DMARC, MX records, DNS security (NEW)
  - Privacy: GDPR compliance, Cookie consent, Privacy policy (NEW)
  - Social: Open Graph, Twitter Cards, Schema.org structured data (NEW)
  - API: API security, CORS, Rate limiting, Auth (NEW)
- **Branded Report Export**: PDF, HTML, Markdown, and Plain Text — all with professional branding
- **Client Profile Manager**: Create and manage client context for richer audits
- **Settings Panel**: Language, theme, API key, white-label branding
- **Export Center**: Format comparison, CLI export guide, branding preview
- **Rust Backend**: 7 Tauri commands for audits, config, reports, and system info
- **Brand Assets**: Color palette, brand guidelines, export stylesheet
- **Documentation**: README, examples (sample client profile, sample report)
- **Toolkit**: 19 audit prompt templates + export CSS template

### Technical

- React 19 + TypeScript with strict mode
- Tailwind CSS v4 for styling
- Vite 7 for development and bundling
- Tauri v2 with plugins: opener, shell, fs
- Rust backend with serde, chrono
