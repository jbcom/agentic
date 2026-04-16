# Agentic Design System — Signal Amber

> **This is the definitive branding guide for ALL Agentic repositories.** All documentation sites MUST follow these specifications to ensure brand consistency across the ecosystem.

## Brand Identity

**Agentic** is the polyglot command center for AI agent orchestration. The visual identity communicates:

- **Warmth & Clarity** - Signal Amber palette, warm stone backgrounds
- **Technical depth** - For developers who care about quality
- **Professionalism** - Enterprise-ready, not experimental

---

## Color Palette

### Primary Colors (Signal Amber)

| Token | Hex | Usage |
|-------|-----|-------|
| `--bg-default` | `#0C0A09` | Page background (warm stone) |
| `--bg-paper` | `#1C1917` | Cards, elevated surfaces |
| `--bg-elevated` | `#292524` | Hover states, highlighted areas |
| `--primary` | `#F59E0B` | Primary actions, links, accents |
| `--primary-light` | `#FCD34D` | Hover states |
| `--primary-dark` | `#D97706` | Active/pressed states |
| `--secondary` | `#F59E0B` | Secondary actions (amber family) |

### Text Colors

| Token | Hex | Usage |
|-------|-----|-------|
| `--text-primary` | `#FAFAF9` | Headings, primary text |
| `--text-secondary` | `#A8A29E` | Body text, descriptions |
| `--text-disabled` | `#78716C` | Disabled elements |

### Semantic Colors

| Token | Hex | Usage |
|-------|-----|-------|
| `--success` | `#10b981` | Success states |
| `--warning` | `#f59e0b` | Warnings |
| `--error` | `#ef4444` | Errors |
| `--info` | `#F59E0B` | Informational (uses amber) |

---

## Typography

### Font Stack

```css
:root {
  --font-heading: 'Instrument Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  --font-body: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  --font-mono: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
}
```

### Font Loading

Include these fonts in all documentation pages:

```html
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Instrument+Sans:wght@400;500;600;700&family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
```

### Type Scale

| Element | Font Family | Size | Weight | Line Height |
|---------|-------------|------|--------|-------------|
| H1 | Instrument Sans | 2.5rem (40px) | 700 | 1.2 |
| H2 | Instrument Sans | 2rem (32px) | 700 | 1.2 |
| H3 | Instrument Sans | 1.5rem (24px) | 600 | 1.3 |
| H4 | Instrument Sans | 1.25rem (20px) | 600 | 1.3 |
| Body | Inter | 1rem (16px) | 400 | 1.6 |
| Code | JetBrains Mono | 0.875rem (14px) | 400 | 1.5 |

---

## CSS Variables Reference

Complete set of CSS variables for documentation sites:

```css
:root {
  /* Backgrounds - Warm Stone */
  --bg-default: #0C0A09;
  --bg-paper: #1C1917;
  --bg-elevated: #292524;

  /* Primary - Amber */
  --primary: #F59E0B;
  --primary-light: #FCD34D;
  --primary-dark: #D97706;

  /* Text */
  --text-primary: #FAFAF9;
  --text-secondary: #A8A29E;
  --text-disabled: #78716C;

  /* Borders */
  --border: #292524;

  /* Semantic */
  --success: #10b981;
  --warning: #f59e0b;
  --error: #ef4444;
  --info: #F59E0B;

  /* Typography */
  --font-heading: 'Instrument Sans', -apple-system, sans-serif;
  --font-body: 'Inter', -apple-system, sans-serif;
  --font-mono: 'JetBrains Mono', monospace;
}
```

---

## Accessibility

### Contrast Ratios

All text/background combinations meet WCAG AA (4.5:1):

| Foreground | Background | Passes |
|------------|------------|--------|
| `#FAFAF9` | `#0C0A09` | Yes |
| `#A8A29E` | `#0C0A09` | Yes |
| `#F59E0B` | `#0C0A09` | Yes |

### Focus States

```css
*:focus-visible {
  outline: 2px solid var(--primary);
  outline-offset: 2px;
}
```

### Reduced Motion

```css
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
}
```

---

## Implementation Checklist

When setting up documentation for any Agentic repository:

- [ ] All CSS custom properties use Signal Amber values
- [ ] Fonts loading correctly (Instrument Sans, Inter, JetBrains Mono)
- [ ] Background uses warm stone palette (`#0C0A09`)
- [ ] Primary accent is amber `#F59E0B`
- [ ] Code blocks use `--bg-paper` background
- [ ] Typography scale matches specification
- [ ] Sidebar navigation styled consistently
- [ ] Focus states visible for accessibility
- [ ] Contrast ratios meet WCAG AA
- [ ] Reduced motion respected

> **Note:** The old jbcom dark theme (cyan `#06b6d4`, background `#0a0f1a`, Space Grotesk headings) is deprecated. All new work must use Signal Amber.
