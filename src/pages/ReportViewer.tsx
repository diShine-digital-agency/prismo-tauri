import { useState, useCallback } from "react";
import Markdown from "react-markdown";

interface ReportViewerProps {
  content: string;
  onBack: () => void;
}

/** Generate a date string for filenames (YYYY-MM-DD). */
function todayStamp(): string {
  return new Date().toISOString().split("T")[0];
}

/**
 * Minimal Markdown → plain text conversion.
 * Strips formatting while preserving structure via line-breaks.
 */
function markdownToPlainText(md: string): string {
  return md
    .replace(/^#{1,6}\s+(.*)/gm, (_, title: string) => `${title}\n${"─".repeat(title.length)}`)
    .replace(/\*\*(.*?)\*\*/g, "$1")
    .replace(/\*(.*?)\*/g, "$1")
    .replace(/`{1,3}(.*?)`{1,3}/g, "$1")
    .replace(/\[([^\]]+)\]\([^)]+\)/g, "$1")
    .replace(/^\|(.+)\|$/gm, (_, row: string) =>
      row.split("|").map((c: string) => c.trim()).filter(Boolean).join("  │  ")
    )
    .replace(/^\|[-:| ]+\|$/gm, "")
    .replace(/^---+$/gm, "────────────────────────────────");
}

/**
 * Escape HTML special characters to prevent XSS when embedding
 * user-provided text into an HTML template.
 */
function escapeHtml(text: string): string {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

/**
 * Convert Markdown to basic HTML for export.
 * All user text is escaped first, then Markdown syntax is converted to
 * safe HTML elements.  Tables are converted to proper <table> elements.
 */
function markdownToHtml(md: string): string {
  // 1. Escape HTML entities in the raw Markdown first to prevent XSS
  let html = escapeHtml(md);

  // 2. Convert Markdown tables before other transforms
  //    Match table blocks: header row, separator row, data rows
  html = html.replace(
    /^(\|.+\|)\n(\|[-:| ]+\|)\n((?:\|.+\|\n?)+)/gm,
    (_match, headerLine: string, _sep: string, bodyBlock: string) => {
      const headers = headerLine.split("|").map((c: string) => c.trim()).filter(Boolean);
      const headerHtml = headers.map((h: string) => `<th>${h}</th>`).join("");

      const rows = bodyBlock.trim().split("\n");
      const bodyHtml = rows
        .map((row: string) => {
          const cells = row.split("|").map((c: string) => c.trim()).filter(Boolean);
          return `<tr>${cells.map((c: string) => `<td>${c}</td>`).join("")}</tr>`;
        })
        .join("\n");

      return `<table><thead><tr>${headerHtml}</tr></thead><tbody>${bodyHtml}</tbody></table>`;
    }
  );

  html = html
    // Headings
    .replace(/^######\s+(.*)/gm, "<h6>$1</h6>")
    .replace(/^#####\s+(.*)/gm, "<h5>$1</h5>")
    .replace(/^####\s+(.*)/gm, "<h4>$1</h4>")
    .replace(/^###\s+(.*)/gm, "<h3>$1</h3>")
    .replace(/^##\s+(.*)/gm, "<h2>$1</h2>")
    .replace(/^#\s+(.*)/gm, "<h1>$1</h1>")
    // Bold and italic
    .replace(/\*\*(.*?)\*\*/g, "<strong>$1</strong>")
    .replace(/\*(.*?)\*/g, "<em>$1</em>")
    // Inline code
    .replace(/`([^`]+)`/g, "<code>$1</code>")
    // Links (only allow http/https/mailto protocols)
    .replace(/\[([^\]]+)\]\(((?:https?|mailto):[^)]+)\)/g, '<a href="$2">$1</a>')
    // Horizontal rules
    .replace(/^---+$/gm, "<hr>")
    // Unordered list items
    .replace(/^- (.+)/gm, "<li>$1</li>")
    // Checkboxes
    .replace(/^✅ (.+)/gm, "<li>✅ $1</li>")
    .replace(/^🔴 (.+)/gm, "<li>🔴 $1</li>")
    .replace(/^🟠 (.+)/gm, "<li>🟠 $1</li>")
    .replace(/^🟡 (.+)/gm, "<li>🟡 $1</li>")
    .replace(/^🟢 (.+)/gm, "<li>🟢 $1</li>")
    // Line breaks for paragraphs
    .replace(/\n\n/g, "</p><p>")
    .replace(/\n/g, "<br>");

  // Wrap in paragraph if not already wrapped in a block element
  if (!html.startsWith("<h") && !html.startsWith("<p") && !html.startsWith("<table")) {
    html = `<p>${html}</p>`;
  }

  return html;
}

export default function ReportViewer({ content, onBack }: ReportViewerProps) {
  const [exportError, setExportError] = useState<string | null>(null);

  const downloadBlob = useCallback((blob: Blob, filename: string) => {
    const url = URL.createObjectURL(blob);
    try {
      const a = document.createElement("a");
      a.href = url;
      a.download = filename;
      a.click();
    } finally {
      URL.revokeObjectURL(url);
    }
  }, []);

  const handleExportMarkdown = useCallback(() => {
    try {
      setExportError(null);
      const blob = new Blob([content], { type: "text/markdown" });
      downloadBlob(blob, `prismo-report-${todayStamp()}.md`);
    } catch (e: unknown) {
      setExportError(`Markdown export failed: ${e instanceof Error ? e.message : String(e)}`);
    }
  }, [content, downloadBlob]);

  const handleExportTxt = useCallback(() => {
    try {
      setExportError(null);
      const plainText = markdownToPlainText(content);
      const blob = new Blob([plainText], { type: "text/plain" });
      downloadBlob(blob, `prismo-report-${todayStamp()}.txt`);
    } catch (e: unknown) {
      setExportError(`Text export failed: ${e instanceof Error ? e.message : String(e)}`);
    }
  }, [content, downloadBlob]);

  const handleExportHtml = useCallback(() => {
    try {
      setExportError(null);
      const renderedBody = markdownToHtml(content);
      const htmlContent = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Prismo Audit Report</title>
  <style>
    body { font-family: 'Inter', 'Segoe UI', sans-serif; max-width: 800px; margin: 0 auto; padding: 40px; color: #2D3436; line-height: 1.6; }
    h1 { color: #6C5CE7; border-bottom: 3px solid #6C5CE7; padding-bottom: 8px; }
    h2 { color: #2D3436; border-bottom: 1px solid #DFE6E9; padding-bottom: 6px; margin-top: 24px; }
    h3 { color: #6C5CE7; }
    table { width: 100%; border-collapse: collapse; margin: 16px 0; }
    th { background-color: #6C5CE7; color: white; padding: 10px 12px; text-align: left; }
    td { padding: 8px 12px; border-bottom: 1px solid #DFE6E9; }
    tr:nth-child(even) { background-color: #F5F6FA; }
    code { background-color: #F5F6FA; padding: 2px 6px; border-radius: 3px; color: #E17055; font-family: 'JetBrains Mono', monospace; }
    pre { background-color: #2D3436; color: #F5F6FA; padding: 16px; border-radius: 6px; overflow-x: auto; }
    blockquote { border-left: 4px solid #6C5CE7; padding-left: 16px; color: #636E72; font-style: italic; }
    a { color: #6C5CE7; }
    hr { border: none; border-top: 1px solid #DFE6E9; margin: 24px 0; }
    .footer { text-align: center; margin-top: 40px; padding-top: 16px; border-top: 1px solid #DFE6E9; color: #999; font-size: 12px; }
    @media print {
      @page { margin: 25mm 20mm; }
      body { font-size: 11pt; }
    }
  </style>
</head>
<body>
  <div id="content">${renderedBody}</div>
  <div class="footer">
    Powered by Prismo | diShine Digital Agency | dishine.it
  </div>
</body>
</html>`;

      const blob = new Blob([htmlContent], { type: "text/html" });
      downloadBlob(blob, `prismo-report-${todayStamp()}.html`);
    } catch (e: unknown) {
      setExportError(`HTML export failed: ${e instanceof Error ? e.message : String(e)}`);
    }
  }, [content, downloadBlob]);

  const handlePrint = useCallback(() => {
    window.print();
  }, []);

  return (
    <div className="p-8 max-w-4xl mx-auto">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <button
          onClick={onBack}
          className="text-sm text-gray-400 hover:text-gray-200 flex items-center gap-1 transition-colors"
        >
          ← Back to reports
        </button>
        <div className="flex items-center gap-2">
          <button
            onClick={handleExportMarkdown}
            className="bg-gray-800 hover:bg-gray-700 text-gray-300 px-3 py-2 rounded-lg text-sm transition-colors flex items-center gap-1.5"
            aria-label="Export as Markdown"
          >
            📋 Markdown
          </button>
          <button
            onClick={handleExportTxt}
            className="bg-gray-800 hover:bg-gray-700 text-gray-300 px-3 py-2 rounded-lg text-sm transition-colors flex items-center gap-1.5"
            aria-label="Export as plain text"
          >
            📝 Text
          </button>
          <button
            onClick={handleExportHtml}
            className="bg-gray-800 hover:bg-gray-700 text-gray-300 px-3 py-2 rounded-lg text-sm transition-colors flex items-center gap-1.5"
            aria-label="Export as HTML"
          >
            🌐 HTML
          </button>
          <button
            onClick={handlePrint}
            className="bg-purple-600 hover:bg-purple-700 text-white px-3 py-2 rounded-lg text-sm transition-colors flex items-center gap-1.5"
            aria-label="Print or save as PDF"
          >
            🖨️ Print / PDF
          </button>
        </div>
      </div>

      {/* Export error */}
      {exportError && (
        <div role="alert" className="mb-4 bg-red-900/30 border border-red-500/40 rounded-lg px-4 py-3 text-sm text-red-300">
          {exportError}
        </div>
      )}

      {/* Report Content */}
      <div className="bg-gray-900 border border-gray-800 rounded-xl p-8 prose max-w-none">
        <Markdown>{content}</Markdown>
      </div>
    </div>
  );
}
