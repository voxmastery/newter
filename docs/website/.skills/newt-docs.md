# Newt Documentation Site Skill

## Quality Standard
Industrial grade. Benchmark: cursor.com/docs.

## Design System
- Primary font: Inter (Google Fonts)
- Code font: JetBrains Mono (Google Fonts)
- Base unit: 4px
- Content max-width: 860px
- NavBar height: 60px
- Sidebar width: 260px
- Code block bg: always #0f111a regardless of light/dark mode
- Accent color: Violet #7c3aed (light) / #8b5cf6 (dark)

## Component Specifications

### Search Palette (cursor.com/docs clone)
- Trigger: CMD+K / CTRL+K, also clicking NavBar search button
- Overlay: full viewport, backdrop rgba(0,0,0,0.6) blur(4px)
- Card: centered, max-width 640px, top 15%, border-radius 12px
- Input: 18px, no border, placeholder "Search documentation..."
- Results: grouped by page, with section + snippet per result
- Highlight: matching text in mark with accent bg at 30% opacity
- Keyboard: ArrowUp/Down navigate, Enter open, ESC close
- Footer: keyboard shortcuts left, logo right
- Animation open: 150ms scale(0.96 to 1) + opacity(0 to 1) ease-out
- Animation close: 100ms reverse ease-in
- Recent searches: stored in localStorage key 'newt-docs-recent-searches'
- Suggested pages when empty: Getting Started, Elements, State, CLI

### Code Blocks
- Always dark background: #0f111a
- Border: 1px solid rgba(255,255,255,0.08)
- Border-radius: 8px
- Padding: 20px 24px
- Copy button: top-right icon, shows checkmark 2s after click

### Callout Blocks
- tip: border-left 3px solid #10b981, bg rgba(16,185,129,0.08)
- info: border-left 3px solid #3b82f6, bg rgba(59,130,246,0.08)
- warning: border-left 3px solid #f59e0b, bg rgba(245,158,11,0.08)
- danger: border-left 3px solid #ef4444, bg rgba(239,68,68,0.08)
- All: border-radius 0 8px 8px 0, padding 16px 20px
