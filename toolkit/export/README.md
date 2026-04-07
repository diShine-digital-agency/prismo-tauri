# Prismo Report Export

Export audit reports as branded PDF, HTML, Markdown, or plain text.

📌 **Quick links**: [README](../../README.md) · [User Guide](../../docs/GUIDE.md) · [Brand Guidelines](../../assets/branding/BRAND.md)

## Quick Export (PDF)

```bash
npm install -g md-to-pdf
npx md-to-pdf toolkit/reports/your-report.md --stylesheet toolkit/export/report-style.css
```

## Formats

| Format | Command | Branded |
|--------|---------|---------|
| PDF | `npx md-to-pdf report.md --stylesheet toolkit/export/report-style.css` | ✅ |
| HTML | `npx md-to-pdf report.md --stylesheet toolkit/export/report-style.css --as-html` | ✅ |
| Markdown | Native format | ✅ |
| Plain Text | `sed 's/[#*_\`]//g' report.md > report.txt` | 🟡 |

## In-App Export

You can also export directly from the Prismo Tauri desktop app:

1. Go to **📄 Reports** in the sidebar
2. Click on a report to open it
3. Use the export buttons at the top:
   - 📋 **Markdown** — Download as `.md`
   - 📝 **Text** — Download as `.txt`
   - 🌐 **HTML** — Download as branded `.html`
   - 🖨️ **Print / PDF** — Open print dialog → Save as PDF

> 📖 See the [User Guide — Exporting Reports](../../docs/GUIDE.md#10-exporting-reports) for detailed instructions with screenshots.

## Customizing

- Edit `report-style.css` to change colors, fonts, and branding
- Edit Settings → Branding in the app for white-label use
- See [Brand Guidelines](../../assets/branding/BRAND.md) for the official color palette

## Sample Report

See [docs/examples/sample-report-website-performance.md](../../docs/examples/sample-report-website-performance.md) for an example of a completed audit report.
