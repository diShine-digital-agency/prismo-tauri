# Prismo Tauri — Complete User Guide

> **This guide is for everyone** — whether you're a developer, a digital agency consultant, or someone who has never opened a terminal before. Every step is explained in plain language.

📌 **Quick links**: [README](../README.md) · [Changelog](../CHANGELOG.md) · [Contributing](../CONTRIBUTING.md) · [Security](../SECURITY.md) · [Brand Guidelines](../assets/branding/BRAND.md) · [Export Guide](../toolkit/export/README.md)

---

## Table of Contents

1. [What is Prismo Tauri?](#1-what-is-prismo-tauri)
2. [Who is this for?](#2-who-is-this-for)
3. [System Requirements](#3-system-requirements)
4. [Installation — Step by Step](#4-installation--step-by-step)
   - [4.1 Windows](#41-windows)
   - [4.2 macOS](#42-macos)
   - [4.3 Linux](#43-linux)
5. [First Launch — Your First 5 Minutes](#5-first-launch--your-first-5-minutes)
6. [The Dashboard](#6-the-dashboard)
7. [Running Your First Audit](#7-running-your-first-audit)
8. [Understanding Audit Categories](#8-understanding-audit-categories)
9. [Viewing Reports](#9-viewing-reports)
10. [Exporting Reports](#10-exporting-reports)
    - [10.1 In-App Export](#101-in-app-export)
    - [10.2 CLI Export (Advanced)](#102-cli-export-advanced)
11. [Managing Clients](#11-managing-clients)
12. [Settings & Configuration](#12-settings--configuration)
    - [12.1 Language](#121-language)
    - [12.2 API Key](#122-api-key)
    - [12.3 White-Label / Rebranding](#123-white-label--rebranding)
13. [Project Structure Explained](#13-project-structure-explained)
14. [Building from Source (Developers)](#14-building-from-source-developers)
15. [Troubleshooting](#15-troubleshooting)
16. [FAQ](#16-faq)
17. [Getting Help](#17-getting-help)

---

## 1. What is Prismo Tauri?

**Prismo Tauri** is a desktop application that helps digital agencies and IT consultants run professional audits on websites, systems, and digital infrastructure. Think of it as a "Swiss Army knife" for digital diagnostics.

It provides:

- **19 audit types** covering everything from website speed to GDPR privacy compliance
- **Branded reports** you can export as PDF, HTML, Markdown, or plain text
- **Client management** to keep track of who you're auditing
- **White-label mode** so you can rebrand it with your own agency name

The app runs on **Windows, macOS, and Linux** and is built with [Tauri](https://tauri.app) (a lightweight framework for desktop apps) and React.

> 💡 **New to this?** You don't need to know what React or Tauri are to use Prismo. The app works like any other desktop application — install it, open it, click buttons.

**Related**: For the original command-line version, see the [Prismo CLI](https://github.com/diShine-digital-agency/prismo) repository.

---

## 2. Who is this for?

| You are... | How you'll use Prismo |
|---|---|
| 🏢 **Digital agency consultant** | Run audits for clients, export branded reports |
| 🔧 **IT technician** | Diagnose system health, network issues, security |
| 📈 **SEO specialist** | Technical SEO audits, competitive analysis |
| 🛡️ **Security analyst** | Website security, API security, GDPR compliance |
| 🎓 **Student / Learner** | Learn about web performance, SEO, security concepts |

---

## 3. System Requirements

### Minimum Requirements

| Component | Requirement |
|-----------|-------------|
| **OS** | Windows 10+, macOS 11+, or Linux (Ubuntu 20.04+, Fedora 33+) |
| **RAM** | 4 GB (8 GB recommended) |
| **Disk** | 200 MB free space |
| **Display** | 1280×720 minimum |

### For Building from Source (Developers Only)

| Tool | Version | Download |
|------|---------|----------|
| Node.js | 22 or newer | [nodejs.org](https://nodejs.org) |
| Rust | Latest stable | [rustup.rs](https://rustup.rs) |
| npm | Comes with Node.js | — |

> 💡 **Not a developer?** Skip this section. You only need the pre-built binary (see [Installation](#4-installation--step-by-step)).

---

## 4. Installation — Step by Step

### 4.1 Windows

#### Option A: Pre-built Installer (Recommended)

1. Go to the [Releases page](https://github.com/diShine-digital-agency/prismo-tauri/releases)
2. Download the file ending in `.msi` (e.g., `Prismo_1.1.0_x64_en-US.msi`)
3. Double-click the downloaded file
4. If Windows shows "Windows protected your PC", click **More info** → **Run anyway** (this is normal for unsigned apps)
5. Follow the installer wizard — click **Next** → **Next** → **Install** → **Finish**
6. Launch Prismo from the Start Menu or Desktop shortcut

#### Option B: Build from Source

> ⚠️ This is for developers only. If you just want to use the app, use Option A.

1. **Install Node.js**: Download from [nodejs.org](https://nodejs.org) (LTS version). Run the installer, check all boxes.

2. **Install Rust**: Open PowerShell and run:
   ```powershell
   winget install Rustlang.Rustup
   ```
   Or download from [rustup.rs](https://rustup.rs). Restart your terminal after installation.

3. **Install WebView2** (usually pre-installed on Windows 10/11):
   - Check: Open Edge browser. If it works, you have WebView2.
   - If needed: Download from [developer.microsoft.com/webview2](https://developer.microsoft.com/en-us/microsoft-edge/webview2)

4. **Clone and build**:
   ```powershell
   git clone https://github.com/diShine-digital-agency/prismo-tauri.git
   cd prismo-tauri
   npm install
   npm run tauri build
   ```

5. The built app will be in: `src-tauri\target\release\Prismo.exe`

---

### 4.2 macOS

#### Option A: Pre-built App (Recommended)

1. Go to the [Releases page](https://github.com/diShine-digital-agency/prismo-tauri/releases)
2. Download the `.dmg` file
3. Open the `.dmg` and drag **Prismo** into your **Applications** folder
4. First launch: Right-click → **Open** (macOS blocks unsigned apps on first run)
5. Click **Open** in the dialog

#### Option B: Build from Source

1. **Install Xcode Command Line Tools**:
   ```bash
   xcode-select --install
   ```
   Click "Install" when prompted. Wait for it to finish (~5 min).

2. **Install Node.js**:
   ```bash
   # Using Homebrew (recommended)
   brew install node

   # Or download from nodejs.org
   ```

3. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   When prompted, press **1** (default installation). Then restart your terminal or run:
   ```bash
   source "$HOME/.cargo/env"
   ```

4. **Clone and build**:
   ```bash
   git clone https://github.com/diShine-digital-agency/prismo-tauri.git
   cd prismo-tauri
   npm install
   npm run tauri build
   ```

5. The built app will be in: `src-tauri/target/release/bundle/dmg/`

---

### 4.3 Linux

#### Option A: Pre-built Package (Recommended)

1. Go to the [Releases page](https://github.com/diShine-digital-agency/prismo-tauri/releases)
2. Download the `.deb` (Ubuntu/Debian) or `.AppImage` (universal) file
3. Install:
   ```bash
   # Debian/Ubuntu (.deb)
   sudo dpkg -i prismo_1.1.0_amd64.deb

   # AppImage (any distro)
   chmod +x Prismo_1.1.0_amd64.AppImage
   ./Prismo_1.1.0_amd64.AppImage
   ```

#### Option B: Build from Source

1. **Install system dependencies**:

   **Ubuntu / Debian**:
   ```bash
   sudo apt update
   sudo apt install -y libwebkit2gtk-4.1-dev librsvg2-dev libgtk-3-dev libsoup-3.0-dev build-essential curl wget
   ```

   **Fedora**:
   ```bash
   sudo dnf install webkit2gtk4.1-devel librsvg2-devel gtk3-devel libsoup3-devel gcc gcc-c++
   ```

   **Arch Linux**:
   ```bash
   sudo pacman -S webkit2gtk-4.1 librsvg gtk3 libsoup3 base-devel
   ```

2. **Install Node.js**:
   ```bash
   # Using NodeSource (recommended)
   curl -fsSL https://deb.nodesource.com/setup_22.x | sudo -E bash -
   sudo apt install -y nodejs
   ```

3. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source "$HOME/.cargo/env"
   ```

4. **Clone and build**:
   ```bash
   git clone https://github.com/diShine-digital-agency/prismo-tauri.git
   cd prismo-tauri
   npm install
   npm run tauri build
   ```

5. The built app will be in: `src-tauri/target/release/bundle/`

---

## 5. First Launch — Your First 5 Minutes

When you open Prismo for the first time, here's what to do:

### Step 1: Explore the Dashboard

The **Dashboard** is your home screen. It shows:

- 📁 **9** audit categories
- 🔍 **19** available audits
- 📤 **4** export formats
- 🌐 **3** languages

You'll also see **Quick Start** cards for the most popular audits.

### Step 2: Set Up Your API Key (Optional)

If you want to run AI-powered audits:

1. Click **⚙️ Settings** in the left sidebar
2. Under "AI Engine", enter your **Anthropic API key**
   - Get one at [console.anthropic.com](https://console.anthropic.com)
3. Click **Save Settings**

> 💡 You can explore the app and view sample reports without an API key.

### Step 3: Run a Quick Audit

1. Click **🔍 Run Audit** in the sidebar
2. Select **Website Performance** (the ⚡ card)
3. Enter a URL (e.g., `https://example.com`)
4. Click **🚀 Run Audit**

### Step 4: View the Sample Reports

1. Click **📄 Reports** in the sidebar
2. Click on any sample report to view it
3. Try the export buttons: Markdown, Text, HTML, or Print

---

## 6. The Dashboard

The dashboard gives you an overview of Prismo's capabilities.

| Section | What it shows |
|---------|---------------|
| **Stats** | Number of categories, audits, formats, languages |
| **Quick Start** | Six most popular audit types — click to jump to them |
| **How It Works** | 4-step explanation of the audit workflow |
| **About** | Version info and agency branding |

**Navigation**: Use the **sidebar on the left** to move between pages. The current page is highlighted in purple.

---

## 7. Running Your First Audit

### Step-by-Step

1. **Go to Run Audit** — Click "🔍 Run Audit" in the sidebar
2. **Filter by category** (optional) — Click a category tab at the top (e.g., "SEO", "Security")
3. **Select an audit** — Click any audit card. You'll see:
   - The audit name and description
   - An input field for the target URL (for web-based audits)
   - A "🚀 Run Audit" button
4. **Enter the target** — For web audits, enter the full URL including `https://`
5. **Run it** — Click "🚀 Run Audit" and wait for results
6. **View output** — Results appear in the output panel below

### Tips

- **System health audits** (Windows, Linux, macOS) don't need a URL — they scan your local machine
- **Web audits** need a URL like `https://example.com`
- Click **← Back to audits** to return to the selection screen

> 📖 **Related**: See [Understanding Audit Categories](#8-understanding-audit-categories) for what each audit checks.

---

## 8. Understanding Audit Categories

Prismo includes **19 audits** organized into **9 categories**. Here's what each one does:

### 🖥️ System Health (5 audits)

| Audit | What it checks |
|-------|---------------|
| **Windows System Diagnosis** | CPU, RAM, disk usage, running services, Windows logs, pending updates |
| **Linux System Diagnosis** | OS info, hardware, storage, services, logs, network configuration |
| **macOS System Diagnosis** | APFS volumes, Time Machine, daemons, security settings, performance |
| **Log Analysis** | Parse any log file for errors, warnings, patterns, and anomalies |
| **Network Diagnostics** | Network interfaces, DNS resolution, routing, open ports, firewall rules |

### ⚡ Web & Performance (3 audits)

| Audit | What it checks |
|-------|---------------|
| **Website Performance** | Core Web Vitals (LCP, INP, CLS), Lighthouse scores, page weight |
| **Tech Stack Analysis** | Frameworks, CMS, hosting provider, CDN, third-party scripts |
| **Accessibility Audit** | WCAG 2.1 AA compliance, screen reader compatibility, color contrast |

### 🔎 SEO (3 audits)

| Audit | What it checks |
|-------|---------------|
| **Technical SEO** | robots.txt, sitemaps, canonical tags, schema markup, hreflang, redirects |
| **On-Page SEO** | Title tags, meta descriptions, headings, content quality, internal links |
| **Competitive SEO** | Side-by-side comparison with competitor websites |

### 📈 MarTech & Data (2 audits)

| Audit | What it checks |
|-------|---------------|
| **MarTech Stack** | Google Tag Manager, GA4, tracking pixels, CRM integration, consent tools |
| **Data Quality** | Event tracking accuracy, UTM consistency, data layer validation |

### 🔒 Security (2 audits)

| Audit | What it checks |
|-------|---------------|
| **Website Security** | SSL/TLS certificates, security headers, CMS vulnerabilities, cookie flags |
| **System Security** | Users, file permissions, firewall, SSH config, disk encryption, patching |

### 📧 Email & DNS (1 audit)

| Audit | What it checks |
|-------|---------------|
| **Email & DNS** | SPF, DKIM, DMARC records, MX configuration, DNS security (DNSSEC) |

### 🏛️ Privacy (1 audit)

| Audit | What it checks |
|-------|---------------|
| **GDPR & Privacy** | Cookie consent banner, privacy policy completeness, data collection practices |

### 📱 Social (1 audit)

| Audit | What it checks |
|-------|---------------|
| **Social Media & Structured Data** | Open Graph tags, Twitter Cards, Schema.org markup |

### 🔌 API (1 audit)

| Audit | What it checks |
|-------|---------------|
| **API Security** | Endpoint discovery, authentication, CORS policy, rate limiting, error handling |

> 📖 **Related**: The raw prompt templates are in [`toolkit/prompts/`](../toolkit/prompts/). See the [Export Guide](../toolkit/export/README.md) for how to export results.

---

## 9. Viewing Reports

### Where are reports?

Click **📄 Reports** in the sidebar. You'll see a list of all your audit reports.

### What you can do

- **Search** — Type in the search bar to filter reports by name or filename
- **View** — Click any report to open it in the Report Viewer
- **Export** — From the Report Viewer, use the export buttons (see next section)

### Sample Reports

Prismo comes with **sample reports** so you can see what a real audit looks like before running your own. These are pre-loaded demo data.

> 📖 **Related**: See [docs/examples/](examples/) for sample report and client profile files.

---

## 10. Exporting Reports

### 10.1 In-App Export

When viewing a report, you'll see four export buttons at the top:

| Button | Format | Best for |
|--------|--------|----------|
| 📋 **Markdown** | `.md` file | Technical documentation, GitHub, version control |
| 📝 **Text** | `.txt` file | Universal compatibility, simple text readers |
| 🌐 **HTML** | `.html` file | Email, web embedding, offline viewing in a browser |
| 🖨️ **Print / PDF** | PDF via print dialog | Client delivery, formal presentations |

#### How to export a PDF

1. Open a report in the Report Viewer
2. Click **🖨️ Print / PDF**
3. In the print dialog:
   - Change "Destination" to **Save as PDF**
   - Click **Save**
4. Choose a filename and location

All exported files include **Prismo branding** (colors, footer, styled tables).

### 10.2 CLI Export (Advanced)

For power users who want more control over PDF generation:

```bash
# Install the conversion tool (one time)
npm install -g md-to-pdf

# Convert a report to branded PDF
npx md-to-pdf your-report.md --stylesheet toolkit/export/report-style.css

# Convert to branded HTML
npx md-to-pdf your-report.md --stylesheet toolkit/export/report-style.css --as-html
```

> 📖 **Related**: See the full [Export Guide](../toolkit/export/README.md) for all options. See [Brand Guidelines](../assets/branding/BRAND.md) for color and typography details.

---

## 11. Managing Clients

The **Client Manager** lets you keep track of the companies and individuals you audit. Client data is persisted in your browser's local storage, so your profiles survive page reloads and application restarts.

### Creating a Client Profile

1. Click **👥 Clients** in the sidebar
2. Click **+ New Client**
3. Fill in the fields:
   - **Client Name** *(required)* — Company or person name
   - **Domain** *(required)* — Their website (e.g., `acme.com`)
   - **Industry** — E.g., "E-commerce", "SaaS", "Healthcare"
   - **Tech Stack** — E.g., "WordPress + WooCommerce"
   - **Analytics** — E.g., "GA4 + GTM"
   - **Notes** — Any additional context for audits
   - **Competitors** — Comma-separated list (e.g., `rival1.com, rival2.com`)
4. Click **Save Client**

### Searching Clients

Use the search bar at the top of the client list to filter by name, domain, or industry.

### Editing or Deleting

- Click **Edit** on any client card to modify their info (the domain field is locked during editing to prevent accidental duplication)
- Click **Delete** to remove a client — you will be asked to confirm before the client is removed

> 📖 **Related**: See [docs/examples/sample-client-profile.json](examples/sample-client-profile.json) for an example of what a client profile looks like.

---

## 12. Settings & Configuration

Click **⚙️ Settings** in the sidebar. Your settings are automatically loaded from your last saved configuration.

### 12.1 Language

Choose your preferred language:
- 🇬🇧 **English** (default)
- 🇮🇹 **Italiano**
- 🇫🇷 **Français**

### 12.2 API Key

To run AI-powered audits, you need an Anthropic API key:

1. Go to [console.anthropic.com](https://console.anthropic.com)
2. Sign up or log in
3. Go to **API Keys** → **Create Key**
4. Copy the key (starts with `sk-ant-api...`)
5. Paste it in the **Anthropic API Key** field in Settings
6. Click **Save Settings**

> ⚠️ **Security note**: Your API key is saved in your browser's local storage and is never written to disk configuration files. Never share your browser data. See [SECURITY.md](../SECURITY.md) for details.

### 12.3 White-Label / Rebranding

You can replace "diShine Digital Agency" with your own agency name:

1. In Settings, under **Branding**, change:
   - **Agency Name** — Your company name
   - **Agency Website** — Your website URL
2. Click **Save Settings**
3. All exported reports will now show your agency name instead

This is perfect for consultants who want to deliver branded reports to their clients.

> 📖 **Related**: See [Brand Guidelines](../assets/branding/BRAND.md) for the full color palette and typography specs.

---

## 13. Project Structure Explained

> 💡 This section is for developers and curious users who want to understand how the project is organized.

```
prismo-tauri/
├── src/                        # 🖥️ Frontend (what you see in the app)
│   ├── components/             # Reusable UI pieces
│   │   ├── Sidebar.tsx         #   Left navigation bar
│   │   └── AuditCard.tsx       #   Audit selection cards
│   ├── pages/                  # App screens
│   │   ├── Dashboard.tsx       #   Home screen
│   │   ├── AuditRunner.tsx     #   Audit selection & execution
│   │   ├── Reports.tsx         #   Report list
│   │   ├── ReportViewer.tsx    #   Report display & export
│   │   ├── ClientManager.tsx   #   Client profile CRUD
│   │   ├── Settings.tsx        #   Configuration
│   │   └── ExportCenter.tsx    #   Export format info
│   ├── App.tsx                 # Main app with page routing
│   ├── main.tsx                # Entry point
│   └── index.css               # Global styles & brand colors
│
├── src-tauri/                  # ⚙️ Backend (Rust, runs natively)
│   ├── src/
│   │   ├── lib.rs              #   All Tauri commands
│   │   └── main.rs             #   App entry point
│   ├── Cargo.toml              #   Rust dependencies
│   ├── tauri.conf.json         #   App config (window size, title, etc.)
│   ├── capabilities/           #   Security permissions
│   └── icons/                  #   App icons for all platforms
│
├── toolkit/                    # 🧰 Audit tools
│   ├── prompts/                #   19 AI prompt templates
│   │   ├── system/             #     System health (5 prompts)
│   │   ├── web/                #     Web & performance (3 prompts)
│   │   ├── seo/                #     SEO (3 prompts)
│   │   ├── martech/            #     MarTech & data (2 prompts)
│   │   ├── security/           #     Security (2 prompts)
│   │   ├── email-dns/          #     Email & DNS (1 prompt)
│   │   ├── privacy/            #     GDPR & privacy (1 prompt)
│   │   ├── social/             #     Social media (1 prompt)
│   │   └── api/                #     API security (1 prompt)
│   └── export/                 #   Report export tools
│       ├── report-style.css    #     Branded CSS for PDF export
│       └── README.md           #     Export documentation
│
├── assets/branding/            # 🎨 Brand assets
│   ├── BRAND.md                #   Brand guidelines
│   └── colors.json             #   Color palette (machine-readable)
│
├── docs/                       # 📚 Documentation
│   ├── GUIDE.md                #   ← You are here!
│   └── examples/               #   Sample files
│       ├── sample-client-profile.json
│       └── sample-report-website-performance.md
│
├── README.md                   # Project overview
├── CHANGELOG.md                # Version history
├── CONTRIBUTING.md             # Developer guide
├── SECURITY.md                 # Security policy
├── LICENSE                     # MIT License
├── package.json                # Node.js dependencies
├── tsconfig.json               # TypeScript config
└── vite.config.ts              # Build tool config
```

---

## 14. Building from Source (Developers)

> 📖 **Related**: See [CONTRIBUTING.md](../CONTRIBUTING.md) for code style and contribution guidelines.

### Quick Start

```bash
# 1. Clone the repository
git clone https://github.com/diShine-digital-agency/prismo-tauri.git
cd prismo-tauri

# 2. Install frontend dependencies
npm install

# 3. Start development mode (hot-reload)
npm run tauri dev

# 4. Build for production
npm run tauri build
```

### Available Commands

| Command | What it does |
|---------|-------------|
| `npm install` | Install all frontend dependencies |
| `npm run dev` | Start Vite dev server only (no Tauri) |
| `npm run build` | Build frontend only (TypeScript + Vite) |
| `npm run tauri dev` | Start full app in development mode with hot-reload |
| `npm run tauri build` | Build optimized production binary |

### Development Mode Explained

When you run `npm run tauri dev`:

1. **Vite** starts a local web server on `http://localhost:1420`
2. **Tauri** opens a native window pointing to that server
3. Any changes you make to `.tsx` or `.css` files are **instantly reflected** (hot module reload)
4. Changes to Rust files (`.rs`) trigger an automatic rebuild

### Build Output

After `npm run tauri build`, the compiled binary is in:

| OS | Location |
|----|----------|
| Windows | `src-tauri/target/release/Prismo.exe` |
| macOS | `src-tauri/target/release/bundle/dmg/Prismo_x.x.x_aarch64.dmg` |
| Linux | `src-tauri/target/release/bundle/deb/prismo_x.x.x_amd64.deb` |

---

## 15. Troubleshooting

### "npm install" fails

| Problem | Solution |
|---------|----------|
| `command not found: npm` | Install Node.js from [nodejs.org](https://nodejs.org) |
| Permission errors on Linux/macOS | Don't use `sudo npm install`. Fix permissions: `sudo chown -R $(whoami) ~/.npm` |
| Network errors | Check your internet connection. Try: `npm install --prefer-offline` |

### "npm run tauri dev" fails

| Problem | Solution |
|---------|----------|
| `error: could not find Rust compiler` | Install Rust: [rustup.rs](https://rustup.rs) |
| Missing WebView2 (Windows) | Download from [Microsoft WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2) |
| Missing GTK/WebKit (Linux) | Install: `sudo apt install libwebkit2gtk-4.1-dev librsvg2-dev libgtk-3-dev libsoup-3.0-dev` |
| Port 1420 already in use | Another Prismo instance is running. Close it and try again. |

### "npm run tauri build" fails

| Problem | Solution |
|---------|----------|
| TypeScript errors | Run `npx tsc --noEmit` to see detailed errors |
| Rust compilation errors | Run `cd src-tauri && cargo build` for details |
| Out of memory | Close other apps, or add swap space on Linux |

### App opens but shows a blank screen

1. Open DevTools: **Ctrl+Shift+I** (Windows/Linux) or **Cmd+Option+I** (macOS)
2. Check the Console tab for errors
3. Try: `npm run build` and then `npm run tauri dev` again

### Reports don't export

- Make sure you're viewing a report first (click one from the Reports page)
- Check if your browser allows file downloads (pop-up blocker)
- Try a different export format

> 📖 **Related**: For security-related issues, see [SECURITY.md](../SECURITY.md).

---

## 16. FAQ

### Do I need an API key to use Prismo?

**No.** You can explore the app, view sample reports, and export documents without an API key. The API key is only needed to run AI-powered audits.

### Is my data sent to the cloud?

**No.** Prismo runs entirely on your computer. Audit results and client profiles are stored locally. The only external communication is with the AI API when running audits (if configured).

### Can I use my own branding?

**Yes!** Go to **Settings → Branding** and change the agency name and website. All exported reports will show your branding. See [White-Label / Rebranding](#123-white-label--rebranding).

### What's the difference between Prismo CLI and Prismo Tauri?

| Feature | Prismo CLI | Prismo Tauri |
|---------|-----------|-------------|
| Interface | Command line | Desktop GUI |
| Report export | Markdown only | PDF, HTML, Markdown, Text |
| Client management | Manual | Built-in UI |
| White-label | Config file | Settings panel |
| Best for | USB-based on-site audits | Office/agency workflow |

### Can I run Prismo from a USB drive?

**Yes.** Build the app, copy the binary to a USB drive, and run it from there. No installation required. The app stores its configuration in your user profile directory.

### How do I update Prismo?

1. Go to the [Releases page](https://github.com/diShine-digital-agency/prismo-tauri/releases)
2. Download the latest version
3. Install it over the existing version (your settings are preserved)

> 📖 **Related**: See [CHANGELOG.md](../CHANGELOG.md) for what's new in each version.

---

## 17. Getting Help

| Need | Where to go |
|------|-------------|
| 🐛 **Bug report** | [GitHub Issues](https://github.com/diShine-digital-agency/prismo-tauri/issues) |
| 💡 **Feature request** | [GitHub Issues](https://github.com/diShine-digital-agency/prismo-tauri/issues) |
| 🔒 **Security issue** | See [SECURITY.md](../SECURITY.md) — do NOT open a public issue |
| 🤝 **Contributing** | See [CONTRIBUTING.md](../CONTRIBUTING.md) |
| 📋 **Version history** | See [CHANGELOG.md](../CHANGELOG.md) |
| 🎨 **Brand assets** | See [Brand Guidelines](../assets/branding/BRAND.md) |
| 📤 **Export help** | See [Export Guide](../toolkit/export/README.md) |

---

> Built with ❤️ by [diShine Digital Agency](https://dishine.it)
