# Changelog

All notable changes to Prismo Tauri will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

📌 **Quick links**: [README](README.md) · [User Guide](docs/GUIDE.md) · [Contributing](CONTRIBUTING.md) · [Security](SECURITY.md)

## [1.1.0] — 2026-04-14

### Added

- **Settings persistence**: Application configuration (language, theme, branding, auto-save) is now loaded from saved config on startup and fully persisted across sessions
- **Client data persistence**: Client profiles are stored in `localStorage` and survive page reloads
- **Client search & filter**: Search clients by name, domain, or industry in the Client Manager
- **Delete confirmation**: Client deletion now requires a two-step confirmation to prevent accidental data loss
- **Client validation**: Required fields (name, domain) are enforced; duplicate domains are prevented; domain field is locked during editing
- **Backend audit loading**: AuditRunner fetches the audit list from the Rust backend on mount, with a graceful fallback to defaults
- **Reports backend integration**: Reports page now calls `list_reports`/`read_report` Tauri commands, with graceful fallback to sample data
- **Reports loading state**: Reports page shows loading indicator while fetching from backend
- **URL validation**: AuditRunner validates URL format before running web-targeting audits
- **Print stylesheet**: Proper `@media print` rules for branded report output via browser print dialog
- **Firefox scrollbar**: Cross-browser custom scrollbar support (previously Webkit-only)
- **Centralized version constant**: `src/constants.ts` provides single source of truth for `APP_VERSION`
- **.gitignore**: Added proper `.gitignore` for `node_modules/`, `dist/`, `src-tauri/target/`, and config files

### Improved

- **HTML export quality**: Reports are now converted from Markdown to rendered HTML (headings, bold, italic, code, links, rules) instead of raw escaped text
- **HTML export tables**: Markdown tables are now converted to proper `<table>` HTML elements in exports
- **Plain text export**: Improved Markdown-to-text conversion preserves heading structure with underlines and table columns with separators
- **Sidebar navigation**: "Reports" now correctly highlights when viewing a report; added `aria-current="page"` for active navigation
- **Accessibility**: Added `role="alert"` on error boundaries, `role="main"` on content area, `aria-label` on navigation, export buttons, search inputs, and client action buttons; decorative emojis marked with `aria-hidden`
- **Error boundary**: Added helpful "restart application" hint text for persistent errors
- **CSS variables**: Replaced hardcoded hex values with `var(--prismo-*)` custom properties throughout stylesheets
- **AuditRunner**: Uses a proper `Set`-based lookup for local audit detection instead of fragile string `.includes()` checks; `isRunning` state correctly resets in `finally` block
- **API key security**: API keys are stored in `localStorage` (client-side only) and never sent to the backend config file
- **ExportCenter dynamic branding**: Report branding preview now reads agency name/website from saved Settings config

### Fixed

- **XSS vulnerability in HTML export**: `markdownToHtml()` now escapes all HTML entities before conversion and only allows safe link protocols (http/https/mailto)
- **Settings not loading**: Previously, opening Settings always showed default values — now loads saved configuration from backend
- **Settings not saving all fields**: `theme`, `auto_save_reports`, and `api_key` were ignored in the save handler
- **Settings timer memory leak**: Save confirmation `setTimeout` now uses `useRef` and is properly cleaned up on unmount
- **Client data loss on reload**: Client profiles were held in React state only and lost on navigation or reload
- **Sidebar highlight bug**: Viewing a report showed no active page in the sidebar (report-viewer wasn't mapped to reports)
- **Reports page not using backend**: Reports were entirely hardcoded sample data — now integrates with Tauri `list_reports`/`read_report`
- **ExportCenter hardcoded branding**: Footer showed hardcoded agency name — now reads from config
- **Version string duplicated**: Version was hardcoded in 3 places — now centralized in `constants.ts`

### Technical

- Extended Rust `PrismoConfig` struct with `theme` (String) and `auto_save_reports` (bool) fields
- Rust `get_config()` now loads from `prismo.config.json` if it exists, falling back to built-in defaults
- Created `src/constants.ts` for centralized `APP_VERSION` constant
- Reports page integrates with backend `list_reports` / `read_report` Tauri commands
- ExportCenter loads branding config from Tauri backend
- HTML export includes XSS sanitization via `escapeHtml()` pre-processing
- Version bumped to 1.1.0 across `package.json`, `Cargo.toml`, `tauri.conf.json`, and all UI references

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
