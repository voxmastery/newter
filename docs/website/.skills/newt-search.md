# Newt Search Palette Skill

## Reference implementation
cursor.com/docs search

## Package
@easyops-cn/docusaurus-search-local

## Component structure
- src/theme/SearchBar/index.js — NavBar trigger button
- src/components/SearchPalette/index.js — full palette component
- src/components/SearchPalette/styles.module.css — all styles
- src/theme/Root.js — mounts palette globally
- src/utils/useSearchPalette.js — state management hook

## Behavior checklist
- CMD+K opens (Mac), CTRL+K opens (Windows/Linux)
- ESC closes
- Click outside closes
- Input autofocuses on open
- Results appear as user types
- Results grouped by page with page title as group header
- Each result shows section title + snippet
- Matching text highlighted in results
- ArrowDown/Up navigate results
- Enter navigates to highlighted result
- Recent searches stored in localStorage
- Suggested pages shown when input empty
- Footer shows keyboard shortcuts
- Opening disables body scroll
- Closing restores body scroll
- Animation: open 150ms ease-out, close 100ms ease-in
