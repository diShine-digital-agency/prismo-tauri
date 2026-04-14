import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

interface ReportsProps {
  onViewReport: (content: string) => void;
}

interface ReportMeta {
  filename: string;
  title: string;
  date: string;
  size: number;
}

// Sample reports used as fallback when no backend reports are available
const sampleReports: ReportMeta[] = [
  {
    filename: "website-performance-acme-2026-04-07.md",
    title: "Website Performance - acme.com",
    date: "2026-04-07 14:32",
    size: 4200,
  },
  {
    filename: "seo-technical-acme-2026-03-20.md",
    title: "Technical SEO - acme.com",
    date: "2026-03-20 10:15",
    size: 3800,
  },
];

// Embedded sample content for demo reports (used when backend is unavailable)
const sampleContent: Record<string, string> = {
  "website-performance-acme-2026-04-07.md": `# Website Performance Audit Report

**Client**: Acme Corp
**Domain**: acme.com
**Date**: 2026-04-07
**Audit Type**: Website Performance
**Prismo Version**: 1.0.0

---

## Executive Summary

The website **acme.com** shows moderate performance with several critical optimization opportunities. The overall Lighthouse performance score is **62/100** on mobile and **78/100** on desktop.

---

## Findings

### 1. Lighthouse Scores

| Metric          | Mobile | Desktop | Status |
|-----------------|--------|---------|--------|
| Performance     | 62     | 78      | 🟠 Needs improvement |
| Accessibility   | 89     | 91      | 🟢 Good |
| Best Practices  | 83     | 83      | 🟡 Moderate |
| SEO             | 92     | 95      | 🟢 Good |

### 2. Core Web Vitals

| Metric | Value   | Threshold | Status |
|--------|---------|-----------|--------|
| LCP    | 4.2s    | < 2.5s    | 🔴 Critical |
| INP    | 180ms   | < 200ms   | 🟢 Good |
| CLS    | 0.15    | < 0.1     | 🟡 Needs improvement |

### 3. Page Weight Analysis

| Resource   | Count | Size (compressed) | Status |
|------------|-------|--------------------|--------|
| HTML       | 1     | 45 KB              | 🟢 Good |
| CSS        | 4     | 180 KB             | 🟡 Could reduce |
| JavaScript | 12    | 890 KB             | 🔴 Too heavy |
| Images     | 23    | 2.4 MB             | 🔴 Needs optimization |
| Fonts      | 3     | 120 KB             | 🟢 Good |
| **Total**  | 43    | **3.6 MB**         | 🔴 Over budget |

---

## Recommendations

### 🔴 Critical (Fix Immediately)

1. **Optimize images**: Convert to WebP/AVIF format, add \`srcset\` for responsive images. **Expected LCP improvement: -1.5s**
2. **Reduce JavaScript bundle**: Code-split the main bundle (890KB). **Expected improvement: -400KB**

### 🟠 High Priority

3. **Fix caching headers**: Add \`Cache-Control: public, max-age=31536000\` for static assets.
4. **Switch to Brotli compression**: Reduce transfer size by ~15%.

### 🟡 Medium Priority

5. **Reduce CLS**: Add explicit \`width\` and \`height\` attributes to all images.
6. **Inline critical CSS**: Extract above-fold CSS and inline in \`<head>\`.

---

## Severity Summary

| Severity | Count |
|----------|-------|
| 🔴 Critical | 2 |
| 🟠 High     | 2 |
| 🟡 Medium   | 2 |
| 🟢 Low      | 0 |

---

*Powered by Prismo | diShine Digital Agency | dishine.it*`,
  "seo-technical-acme-2026-03-20.md": `# Technical SEO Audit Report

**Client**: Acme Corp
**Domain**: acme.com
**Date**: 2026-03-20
**Prismo Version**: 1.0.0

---

## Executive Summary

The technical SEO foundation of acme.com requires attention. Missing hreflang tags and duplicate canonical issues are the most critical findings.

## Findings

### Crawlability
- ✅ robots.txt is properly configured
- ✅ XML sitemap exists and is valid
- 🟠 3 pages in sitemap return 301 redirects

### Indexability
- 🔴 Missing hreflang tags (international content detected)
- 🟠 Duplicate canonical tags on 5 product pages
- ✅ No noindex on important pages

### Site Architecture
- ✅ URL structure is clean and consistent
- 🟡 Some pages exceed 4 levels of depth
- ✅ Internal linking is logical

## Severity Summary

| Severity | Count |
|----------|-------|
| 🔴 Critical | 1 |
| 🟠 High     | 2 |
| 🟡 Medium   | 1 |
| 🟢 Low      | 0 |

---

*Powered by Prismo | diShine Digital Agency | dishine.it*`,
};

export default function Reports({ onViewReport }: ReportsProps) {
  const [reports, setReports] = useState<ReportMeta[]>(sampleReports);
  const [searchQuery, setSearchQuery] = useState("");
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | null>(null);

  // Try to load reports from the backend
  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        const backendReports = await invoke<ReportMeta[]>("list_reports", {
          reportsDir: "prismo-reports",
        });
        if (!cancelled && backendReports.length > 0) {
          setReports(backendReports);
        }
      } catch {
        // Backend unavailable — keep sample reports
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    })();
    return () => { cancelled = true; };
  }, []);

  const handleViewReport = async (report: ReportMeta) => {
    // Check for embedded sample content first (demo mode)
    if (sampleContent[report.filename]) {
      onViewReport(sampleContent[report.filename]);
      return;
    }
    // Try reading from backend
    try {
      setLoadError(null);
      const content = await invoke<string>("read_report", {
        baseDir: "prismo-reports",
        filename: report.filename,
      });
      onViewReport(content);
    } catch (e: unknown) {
      setLoadError(`Failed to load report: ${e instanceof Error ? e.message : String(e)}`);
    }
  };

  const filteredReports = reports.filter(
    (r) =>
      r.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      r.filename.toLowerCase().includes(searchQuery.toLowerCase())
  );

  const formatSize = (bytes: number) => {
    if (bytes < 1024) return `${bytes} B`;
    return `${(bytes / 1024).toFixed(1)} KB`;
  };

  return (
    <div className="p-8 max-w-4xl mx-auto">
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-white mb-2">Reports</h1>
        <p className="text-gray-400">View and manage your audit reports.</p>
      </div>

      {/* Search */}
      <div className="mb-6">
        <input
          type="text"
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          placeholder="Search reports..."
          aria-label="Search reports"
          className="w-full bg-gray-900 border border-gray-800 rounded-lg px-4 py-3 text-gray-200 placeholder-gray-600 focus:outline-none focus:border-purple-500"
        />
      </div>

      {/* Loading state */}
      {loading && (
        <div className="text-center py-8 text-gray-500">
          <p>Loading reports…</p>
        </div>
      )}

      {/* Load error */}
      {loadError && (
        <div role="alert" className="mb-4 bg-red-900/30 border border-red-500/40 rounded-lg px-4 py-3 text-sm text-red-300">
          {loadError}
        </div>
      )}

      {/* Report List */}
      {!loading && (
        <div className="space-y-3">
          {filteredReports.map((report) => (
            <button
              key={report.filename}
              onClick={() => handleViewReport(report)}
              className="w-full bg-gray-900 border border-gray-800 rounded-xl p-5 text-left hover:border-purple-500/40 transition-all group"
            >
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-4">
                  <span className="text-2xl" aria-hidden="true">📄</span>
                  <div>
                    <h3 className="font-medium text-gray-200 group-hover:text-purple-400 transition-colors">
                      {report.title}
                    </h3>
                    <p className="text-xs text-gray-500 mt-1">
                      {report.filename} · {formatSize(report.size)} · {report.date}
                    </p>
                  </div>
                </div>
                <span className="text-gray-600 group-hover:text-purple-400 transition-colors" aria-hidden="true">→</span>
              </div>
            </button>
          ))}

          {filteredReports.length === 0 && (
            <div role="status" aria-live="polite" className="text-center py-12 text-gray-500">
              <p className="text-4xl mb-3" aria-hidden="true">📭</p>
              <p>No reports found. Run an audit to generate your first report.</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
