use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// A single audit prompt template available to the user.
#[derive(Serialize, Deserialize, Clone)]
pub struct AuditPrompt {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub filename: String,
}

/// Basic client profile stored alongside the application configuration.
#[derive(Serialize, Deserialize, Clone)]
pub struct ClientProfile {
    pub name: String,
    pub domain: String,
    pub industry: String,
    pub notes: String,
}

/// Top-level application configuration persisted to disk.
#[derive(Serialize, Deserialize, Clone)]
pub struct PrismoConfig {
    pub version: String,
    pub language: String,
    pub default_report_format: String,
    pub theme: String,
    pub auto_save_reports: bool,
    pub branding: BrandingConfig,
    pub client: ClientProfile,
}

/// Branding settings for white-label report generation.
#[derive(Serialize, Deserialize, Clone)]
pub struct BrandingConfig {
    pub agency: String,
    pub website: String,
}

/// Metadata for a saved report file (used in the report listing).
#[derive(Serialize, Deserialize)]
pub struct ReportMeta {
    pub filename: String,
    pub title: String,
    pub date: String,
    pub size: u64,
}

/// Validate that `requested` resolves to a path inside `base_dir`.
///
/// Both paths must already exist on disk (this calls `canonicalize`).
/// Returns the canonicalized path on success or an error string when
/// a path-traversal attempt (e.g. `../`) is detected.
fn validate_path_inside(base_dir: &Path, requested: &Path) -> Result<PathBuf, String> {
    let canonical_base = base_dir
        .canonicalize()
        .map_err(|e| format!("Invalid base directory: {}", e))?;
    let canonical_requested = requested
        .canonicalize()
        .map_err(|e| format!("Invalid path: {}", e))?;
    if !canonical_requested.starts_with(&canonical_base) {
        return Err("Access denied: path is outside the allowed directory".to_string());
    }
    Ok(canonical_requested)
}

/// Convert a report filename into a human-readable title.
fn filename_to_title(filename: &str) -> String {
    filename
        .trim_end_matches(".md")
        .replace('-', " ")
        .replace('_', " ")
}

/// Returns all 19 built-in audit prompt definitions.
///
/// Each prompt includes an id, display name, category, short description,
/// and the relative path to the Markdown prompt template under `toolkit/prompts/`.
#[tauri::command]
fn get_audit_prompts() -> Vec<AuditPrompt> {
    vec![
        AuditPrompt { id: "windows-health".into(), name: "Windows System Diagnosis".into(), category: "System Health".into(), description: "CPU, RAM, disk, services, logs, pending updates".into(), filename: "system/windows-health.md".into() },
        AuditPrompt { id: "linux-health".into(), name: "Linux System Diagnosis".into(), category: "System Health".into(), description: "OS, hardware, storage, services, logs, network".into(), filename: "system/linux-health.md".into() },
        AuditPrompt { id: "macos-health".into(), name: "macOS System Diagnosis".into(), category: "System Health".into(), description: "APFS, Time Machine, daemons, security, performance".into(), filename: "system/macos-health.md".into() },
        AuditPrompt { id: "log-analysis".into(), name: "Log Analysis".into(), category: "System Health".into(), description: "Parse any log file for errors, warnings, and patterns".into(), filename: "system/log-analysis.md".into() },
        AuditPrompt { id: "network-diagnosis".into(), name: "Network Diagnostics".into(), category: "System Health".into(), description: "Interfaces, DNS, routing, ports, firewall, connectivity".into(), filename: "system/network-diagnosis.md".into() },
        AuditPrompt { id: "website-performance".into(), name: "Website Performance".into(), category: "Web & Performance".into(), description: "Core Web Vitals (LCP, INP, CLS), Lighthouse metrics".into(), filename: "web/website-performance.md".into() },
        AuditPrompt { id: "tech-stack".into(), name: "Tech Stack Analysis".into(), category: "Web & Performance".into(), description: "Frameworks, CMS, hosting, CDN, third-party scripts".into(), filename: "web/tech-stack-analysis.md".into() },
        AuditPrompt { id: "accessibility".into(), name: "Accessibility Audit".into(), category: "Web & Performance".into(), description: "WCAG 2.1 AA compliance checks".into(), filename: "web/accessibility-audit.md".into() },
        AuditPrompt { id: "seo-technical".into(), name: "Technical SEO".into(), category: "SEO".into(), description: "robots.txt, sitemaps, canonicals, schema, hreflang, redirects".into(), filename: "seo/seo-technical.md".into() },
        AuditPrompt { id: "seo-onpage".into(), name: "On-Page SEO".into(), category: "SEO".into(), description: "Titles, metas, headings, content quality, internal links".into(), filename: "seo/seo-onpage.md".into() },
        AuditPrompt { id: "seo-competitive".into(), name: "Competitive SEO".into(), category: "SEO".into(), description: "Side-by-side SEO comparison against competitors".into(), filename: "seo/seo-competitive.md".into() },
        AuditPrompt { id: "martech-stack".into(), name: "MarTech Stack".into(), category: "MarTech & Data".into(), description: "GTM, GA4, pixels, CRM, consent management".into(), filename: "martech/martech-stack-audit.md".into() },
        AuditPrompt { id: "data-quality".into(), name: "Data Quality".into(), category: "MarTech & Data".into(), description: "Event tracking, UTM consistency, data layer validation".into(), filename: "martech/martech-data-quality.md".into() },
        AuditPrompt { id: "website-security".into(), name: "Website Security".into(), category: "Security".into(), description: "SSL/TLS, security headers, CMS vulnerabilities, cookie flags".into(), filename: "security/website-security.md".into() },
        AuditPrompt { id: "system-security".into(), name: "System Security".into(), category: "Security".into(), description: "Users, permissions, firewall, SSH, encryption, patching".into(), filename: "security/system-security.md".into() },
        AuditPrompt { id: "email-dns".into(), name: "Email & DNS Audit".into(), category: "Email & DNS".into(), description: "SPF, DKIM, DMARC, MX records, DNS security".into(), filename: "email-dns/email-dns-audit.md".into() },
        AuditPrompt { id: "gdpr-privacy".into(), name: "GDPR & Privacy".into(), category: "Privacy".into(), description: "Cookie consent, privacy policy, data collection compliance".into(), filename: "privacy/gdpr-privacy-audit.md".into() },
        AuditPrompt { id: "social-media".into(), name: "Social Media & Structured Data".into(), category: "Social".into(), description: "Open Graph, Twitter Cards, Schema.org markup".into(), filename: "social/social-media-audit.md".into() },
        AuditPrompt { id: "api-security".into(), name: "API Security".into(), category: "API".into(), description: "Endpoints, auth, CORS, rate limiting, error handling".into(), filename: "api/api-security-audit.md".into() },
    ]
}

/// Returns the default application configuration.
///
/// If a saved configuration exists at `prismo.config.json` it is loaded
/// and returned instead of the built-in defaults.
#[tauri::command]
fn get_config() -> PrismoConfig {
    if let Ok(data) = fs::read_to_string("prismo.config.json") {
        if let Ok(config) = serde_json::from_str::<PrismoConfig>(&data) {
            return config;
        }
    }
    PrismoConfig {
        version: "1.1.0".into(),
        language: "en".into(),
        default_report_format: "markdown".into(),
        theme: "dark".into(),
        auto_save_reports: true,
        branding: BrandingConfig {
            agency: "diShine Digital Agency".into(),
            website: "https://dishine.it".into(),
        },
        client: ClientProfile {
            name: String::new(),
            domain: String::new(),
            industry: String::new(),
            notes: String::new(),
        },
    }
}

/// Persist the application configuration to `prismo.config.json`.
#[tauri::command]
fn save_config(config: PrismoConfig) -> Result<(), String> {
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write("prismo.config.json", json).map_err(|e| e.to_string())?;
    Ok(())
}

/// List Markdown report files inside `reports_dir`.
///
/// Returns an empty list when the directory does not exist.  Only `.md`
/// files are included.  Results are sorted newest-first by modification date.
#[tauri::command]
fn list_reports(reports_dir: String) -> Result<Vec<ReportMeta>, String> {
    let base_path = PathBuf::from(&reports_dir);

    // Ensure the directory exists; return empty list otherwise.
    if !base_path.is_dir() {
        return Ok(Vec::new());
    }

    // Canonicalize once so we can validate every entry stays inside it.
    let canonical_base = base_path
        .canonicalize()
        .map_err(|e| format!("Invalid reports directory: {}", e))?;

    let mut reports = Vec::new();

    let entries = fs::read_dir(&canonical_base)
        .map_err(|e| format!("Failed to read reports directory: {}", e))?;

    for entry in entries.flatten() {
        let file_path = entry.path();

        // Only consider Markdown files that resolve inside the base dir.
        if file_path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        if let Ok(canon) = file_path.canonicalize() {
            if !canon.starts_with(&canonical_base) {
                continue; // skip symlinks pointing outside
            }
        }

        if let Ok(metadata) = fs::metadata(&file_path) {
            let filename = file_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            let title = filename_to_title(&filename);

            let date = metadata
                .modified()
                .map(|t| {
                    let datetime: chrono::DateTime<chrono::Utc> = t.into();
                    datetime.format("%Y-%m-%d %H:%M").to_string()
                })
                .unwrap_or_else(|_| "Unknown".into());

            reports.push(ReportMeta {
                filename,
                title,
                date,
                size: metadata.len(),
            });
        }
    }

    reports.sort_by(|a, b| b.date.cmp(&a.date));
    Ok(reports)
}

/// Read a report file and return its content.
///
/// The `base_dir` is the reports directory that the frontend is aware of.
/// `filename` is a plain filename (no path separators allowed) inside that
/// directory.  Path traversal attempts are rejected.
#[tauri::command]
fn read_report(base_dir: String, filename: String) -> Result<String, String> {
    // Reject filenames that contain path separators or traversal sequences.
    if filename.contains('/') || filename.contains('\\') || filename.contains("..") {
        return Err("Invalid filename".to_string());
    }

    let base = PathBuf::from(&base_dir);
    let target = base.join(&filename);
    let validated = validate_path_inside(&base, &target)?;

    fs::read_to_string(&validated).map_err(|e| format!("Failed to read report: {}", e))
}

/// Save report content to a file.
///
/// `base_dir` must already exist.  `filename` is a plain filename (no path
/// separators) that will be written inside `base_dir`.  The function creates
/// the base directory if it does not exist yet.
#[tauri::command]
fn save_report(base_dir: String, filename: String, content: String) -> Result<(), String> {
    if filename.contains('/') || filename.contains('\\') || filename.contains("..") {
        return Err("Invalid filename".to_string());
    }

    let base = PathBuf::from(&base_dir);
    fs::create_dir_all(&base).map_err(|e| format!("Failed to create directory: {}", e))?;

    // The file doesn't exist yet so we can't use canonicalize() on it.
    // Instead we canonicalize the base dir and join the (already-validated)
    // plain filename — the separator/traversal check above guarantees that
    // `join` cannot escape `canonical_base`.
    let canonical_base = base
        .canonicalize()
        .map_err(|e| format!("Invalid base directory: {}", e))?;
    let canonical_target = canonical_base.join(&filename);

    if !canonical_target.starts_with(&canonical_base) {
        return Err("Access denied: path is outside the allowed directory".to_string());
    }

    fs::write(&canonical_target, &content)
        .map_err(|e| format!("Failed to save report: {}", e))
}

/// Returns basic OS information (os, arch, family).
#[tauri::command]
fn get_system_info() -> serde_json::Value {
    serde_json::json!({
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "family": std::env::consts::FAMILY,
    })
}

/// Initialise and run the Tauri application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            get_audit_prompts,
            get_config,
            save_config,
            list_reports,
            read_report,
            save_report,
            get_system_info,
        ])
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            eprintln!("Fatal: failed to start Prismo — {}", e);
            std::process::exit(1);
        });
}
