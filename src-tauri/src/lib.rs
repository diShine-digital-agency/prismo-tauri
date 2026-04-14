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
        .replace(['-', '_'], " ")
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

// ──────────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // ── filename_to_title ────────────────────────────────────────────────────

    #[test]
    fn title_strips_md_extension() {
        assert_eq!(filename_to_title("report.md"), "report");
    }

    #[test]
    fn title_replaces_dashes_and_underscores() {
        assert_eq!(
            filename_to_title("seo-technical-acme-2026.md"),
            "seo technical acme 2026"
        );
        assert_eq!(
            filename_to_title("my_report_v2.md"),
            "my report v2"
        );
    }

    #[test]
    fn title_handles_no_extension() {
        assert_eq!(filename_to_title("report"), "report");
    }

    #[test]
    fn title_handles_empty_string() {
        assert_eq!(filename_to_title(""), "");
    }

    // ── validate_path_inside ────────────────────────────────────────────────

    #[test]
    fn validate_path_inside_accepts_child() {
        let dir = std::env::temp_dir().join("prismo_test_validate");
        let _ = fs::create_dir_all(&dir);
        let file = dir.join("test.md");
        fs::write(&file, "content").unwrap();

        let result = validate_path_inside(&dir, &file);
        assert!(result.is_ok());

        // Cleanup
        let _ = fs::remove_file(&file);
        let _ = fs::remove_dir(&dir);
    }

    #[test]
    fn validate_path_inside_rejects_outside() {
        let dir = std::env::temp_dir().join("prismo_test_outside");
        let _ = fs::create_dir_all(&dir);
        // Try to escape to the temp root
        let outside = std::env::temp_dir().join("prismo_test_validate_outside_marker.txt");
        fs::write(&outside, "x").unwrap();

        let result = validate_path_inside(&dir, &outside);
        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(
            err_msg.contains("outside") || err_msg.contains("denied") || err_msg.contains("Invalid"),
            "Unexpected error: {}",
            err_msg
        );

        let _ = fs::remove_file(&outside);
        let _ = fs::remove_dir(&dir);
    }

    #[test]
    fn validate_path_inside_rejects_nonexistent() {
        let dir = std::env::temp_dir().join("prismo_test_noexist");
        let _ = fs::create_dir_all(&dir);
        let fake = dir.join("does_not_exist.md");

        let result = validate_path_inside(&dir, &fake);
        assert!(result.is_err());

        let _ = fs::remove_dir(&dir);
    }

    // ── get_audit_prompts ───────────────────────────────────────────────────

    #[test]
    fn audit_prompts_returns_19_items() {
        let prompts = get_audit_prompts();
        assert_eq!(prompts.len(), 19);
    }

    #[test]
    fn audit_prompts_have_unique_ids() {
        let prompts = get_audit_prompts();
        let mut ids: Vec<&str> = prompts.iter().map(|p| p.id.as_str()).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), 19);
    }

    #[test]
    fn audit_prompts_all_have_filenames() {
        let prompts = get_audit_prompts();
        for p in &prompts {
            assert!(!p.filename.is_empty(), "Prompt {} has empty filename", p.id);
            assert!(p.filename.ends_with(".md"), "Prompt {} filename doesn't end with .md", p.id);
        }
    }

    #[test]
    fn audit_prompts_cover_all_categories() {
        let prompts = get_audit_prompts();
        let categories: std::collections::HashSet<&str> =
            prompts.iter().map(|p| p.category.as_str()).collect();
        let expected = vec![
            "System Health", "Web & Performance", "SEO", "MarTech & Data",
            "Security", "Email & DNS", "Privacy", "Social", "API",
        ];
        for cat in expected {
            assert!(categories.contains(cat), "Missing category: {}", cat);
        }
    }

    // ── get_config ──────────────────────────────────────────────────────────

    #[test]
    fn default_config_has_correct_version() {
        let config = get_config();
        assert_eq!(config.version, "1.1.0");
    }

    #[test]
    fn default_config_has_branding() {
        let config = get_config();
        assert!(!config.branding.agency.is_empty());
        assert!(!config.branding.website.is_empty());
    }

    #[test]
    fn default_config_theme_is_dark() {
        let config = get_config();
        assert_eq!(config.theme, "dark");
    }

    #[test]
    fn default_config_auto_save_is_true() {
        let config = get_config();
        assert!(config.auto_save_reports);
    }

    // ── save_config / get_config roundtrip ──────────────────────────────────

    #[test]
    fn config_serialization_roundtrip() {
        let config = PrismoConfig {
            version: "1.1.0".into(),
            language: "it".into(),
            default_report_format: "html".into(),
            theme: "light".into(),
            auto_save_reports: false,
            branding: BrandingConfig {
                agency: "Test Agency".into(),
                website: "https://test.com".into(),
            },
            client: ClientProfile {
                name: "Test".into(),
                domain: "test.com".into(),
                industry: "Tech".into(),
                notes: "Note".into(),
            },
        };
        let json = serde_json::to_string(&config).unwrap();
        let parsed: PrismoConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.language, "it");
        assert_eq!(parsed.theme, "light");
        assert!(!parsed.auto_save_reports);
        assert_eq!(parsed.branding.agency, "Test Agency");
    }

    // ── read_report ─────────────────────────────────────────────────────────

    #[test]
    fn read_report_rejects_path_traversal() {
        let result = read_report("reports".into(), "../etc/passwd".into());
        assert!(result.is_err());
    }

    #[test]
    fn read_report_rejects_slashes_in_filename() {
        let result = read_report("reports".into(), "subdir/file.md".into());
        assert!(result.is_err());
    }

    #[test]
    fn read_report_rejects_backslash() {
        let result = read_report("reports".into(), "sub\\file.md".into());
        assert!(result.is_err());
    }

    // ── save_report ─────────────────────────────────────────────────────────

    #[test]
    fn save_report_rejects_path_traversal() {
        let result = save_report("reports".into(), "../evil.md".into(), "bad".into());
        assert!(result.is_err());
    }

    #[test]
    fn save_report_rejects_slashes() {
        let result = save_report("reports".into(), "sub/file.md".into(), "bad".into());
        assert!(result.is_err());
    }

    #[test]
    fn save_report_writes_and_reads_back() {
        let dir = std::env::temp_dir().join("prismo_test_save_report");
        let _ = fs::remove_dir_all(&dir);

        let dir_str = dir.to_str().unwrap().to_string();
        let result = save_report(dir_str.clone(), "test-report.md".into(), "# Test\nHello".into());
        assert!(result.is_ok(), "save_report failed: {:?}", result);

        let content = read_report(dir_str, "test-report.md".into());
        assert!(content.is_ok());
        assert_eq!(content.unwrap(), "# Test\nHello");

        let _ = fs::remove_dir_all(&dir);
    }

    // ── list_reports ────────────────────────────────────────────────────────

    #[test]
    fn list_reports_empty_for_nonexistent_dir() {
        let result = list_reports("/tmp/prismo_nonexistent_dir_xyz".into());
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn list_reports_finds_md_files() {
        let dir = std::env::temp_dir().join("prismo_test_list_reports");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("report1.md"), "# Report 1").unwrap();
        fs::write(dir.join("report2.md"), "# Report 2").unwrap();
        fs::write(dir.join("ignore.txt"), "not a report").unwrap();

        let result = list_reports(dir.to_str().unwrap().into());
        assert!(result.is_ok());
        let reports = result.unwrap();
        assert_eq!(reports.len(), 2);
        assert!(reports.iter().all(|r| r.filename.ends_with(".md")));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn list_reports_ignores_non_md_files() {
        let dir = std::env::temp_dir().join("prismo_test_list_ignore");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("data.json"), "{}").unwrap();
        fs::write(dir.join("notes.txt"), "note").unwrap();

        let result = list_reports(dir.to_str().unwrap().into());
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());

        let _ = fs::remove_dir_all(&dir);
    }

    // ── get_system_info ─────────────────────────────────────────────────────

    #[test]
    fn system_info_has_expected_fields() {
        let info = get_system_info();
        assert!(info.get("os").is_some());
        assert!(info.get("arch").is_some());
        assert!(info.get("family").is_some());
    }
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
