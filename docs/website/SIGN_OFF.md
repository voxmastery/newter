# Newt Documentation Website — QA Sign-Off

Date: 2026-03-14
QA Engineer: Principal QA Agent

## Final Scores

### Section A — Search Palette: 20/20 (10/10 each)
- A1: 10/10 — `useSearchPalette.js:23` `e.metaKey && e.key === 'k'`
- A2: 10/10 — `useSearchPalette.js:23` `e.ctrlKey` in same conditional
- A3: 10/10 — `useSearchPalette.js:27` `e.key === 'Escape' && isOpen`
- A4: 10/10 — `SearchPalette/index.js:158` overlay `onClick={close}`, palette `e.stopPropagation()`
- A5: 10/10 — `SearchPalette/index.js:83` `setTimeout(() => inputRef.current?.focus(), 50)` in `useEffect([isOpen])`
- A6: 10/10 — `SearchPalette/index.js:172` `onChange={(e) => setQuery(e.target.value)}` — no debounce
- A7: 10/10 — `SearchPalette/index.js:226-265` results grouped by `item.section` with `.groupHeader` per group
- A8: 10/10 — `SearchPalette/index.js:39-52` `highlightMatch()` wraps matches in `<mark className={styles.highlight}>`
- A9: 10/10 — `SearchPalette/index.js:125-127` ArrowDown with `e.preventDefault()` and `Math.min` bounds
- A10: 10/10 — `SearchPalette/index.js:128-130` ArrowUp with `e.preventDefault()` and `Math.max` bounds
- A11: 10/10 — `SearchPalette/index.js:131-133` Enter calls `navigateTo()` which calls `history.push()`
- A12: 10/10 — `SearchPalette/index.js:7` key `'newt-docs-recent-searches'`, `addRecentSearch()` saves max 5
- A13: 10/10 — `SearchPalette/index.js:10-15` `SUGGESTED_PAGES` array with 4 pages shown when empty
- A14: 10/10 — `SearchPalette/index.js:262-268` footer bar with `↑↓ Navigate`, `↵ Open`, `ESC Close`
- A15: 10/10 — `useSearchPalette.js:12` `document.documentElement.style.overflow = 'hidden'` on open
- A16: 10/10 — `SearchPalette/index.js:2` `import {createPortal} from 'react-dom'`, renders to `document.body`
- A17: 10/10 — `SearchPalette/styles.module.css:36-44` `paletteIn` keyframe 150ms ease-out, scale(0.96)→scale(1)
- A18: 10/10 — `useSearchPalette.js:21-28` `isClosing` state with 100ms delay; `styles.module.css` `paletteOut` keyframe 100ms ease-in
- A19: 10/10 — `SearchPalette/styles.module.css:6-8` `rgba(0,0,0,0.6)`, `backdrop-filter: blur(4px)`, `-webkit-backdrop-filter`
- A20: 10/10 — `SearchPalette/styles.module.css:24,33,27` `max-width: 640px`, `padding-top: 15vh`, `border-radius: 12px`

### Section B — Visual Design: 10/10 (10/10 each)
- B1: 10/10 — All 13 custom color properties in `:root` have matching entries in `[data-theme='dark']` (`custom.css:84-97`)
- B2: 10/10 — `custom.css:68` `--code-bg: #0f111a` in `:root`, `.prism-code` uses `var(--code-bg)` — same in both modes
- B3: 10/10 — `custom.css:1` Google Fonts import with Inter 400-800; `custom.css:42` `--font-sans: 'Inter'`
- B4: 10/10 — `custom.css:1` JetBrains Mono 400-600; `custom.css:43` `--font-mono`; `custom.css:153` `pre, code { font-family: var(--font-mono) }`
- B5: 10/10 — `custom.css:55` `--ifm-navbar-height: 60px`
- B6: 10/10 — `custom.css:56` `--doc-sidebar-width: 260px !important`
- B7: 10/10 — `custom.css:206` `.theme-doc-markdown { max-width: 860px }`
- B8: 10/10 — `custom.css:183-187` `.menu__link--active` with `color: var(--color-accent)`, `border-left: 3px solid`, `background: var(--color-accent-muted)`
- B9: 10/10 — Hero stacks at 768px, features grid at 768px/480px, no fixed widths > 375px outside media queries
- B10: 10/10 — `custom.css:218-231` `.alert--success/info/warning/danger` with `border-left: 3px solid`, `border-radius: 0 8px 8px 0`

### Section C — Content Completeness: 10/10 (10/10 each)
- C1: 10/10 — `ast.rs` has 58 ElementKind variants; all docs/homepage show "58"
- C2: 10/10 — `elements.md` documents all 58 elements across 6 categories with code examples per group
- C3: 10/10 — `props.md` documents all PropName enum variants plus Ident-based props with types and defaults
- C4: 10/10 — 18/18 .md files have `## Next steps` (grep count matches file count)
- C5: 10/10 — `getting-started/index.md` is ~200 words (3-step quickstart, no fluff)
- C6: 10/10 — grep for TODO/Lorem/placeholder/YOUR_USERNAME/coming soon returns zero real matches
- C7: 10/10 — All `.newt` code blocks use valid elements from ElementKind and valid syntax
- C8: 10/10 — `state.md` covers counter (increment/decrement/reset), form (submitted flag), and conditional rendering
- C9: 10/10 — `string-interpolation.md` covers basic, expression, multiple, escaped braces, and prop context
- C10: 10/10 — `cli.md` documents all 5 commands (run/serve/build/check/watch) with all flags and types

### Section D — Syntax Highlighting: 5/5 (10/10 each)
- D1: 10/10 — `prism-newt.js` tag pattern lists all 58 ElementKind names
- D2: 10/10 — `prism-newt.js` attr-name pattern lists all PropName variants with `(?=\s*:)` lookahead
- D3: 10/10 — Token order: comment, string, constant, boolean, keyword, tag, attr-name, function, number, operator, punctuation
- D4: 10/10 — `prism-newt.js` string token has nested `interpolation` pattern for `{expr}`
- D5: 10/10 — `Root.js:3` imports `prism-newt.js`; `docusaurus.config.js` does not list `newt` in `additionalLanguages`

### Section E — Performance & Build: 5/5 (10/10 each)
- E1: 10/10 — `npm run build` outputs `[SUCCESS] Generated static files in "build".` with zero errors
- E2: 10/10 — Build output has zero warning lines
- E3: 10/10 — No broken link warnings in build output (`onBrokenLinks: 'throw'` in config)
- E4: 10/10 — Build generates 20+ pages (18 docs + homepage + 404 + search)
- E5: 10/10 — `package.json` has `start`, `build`, `serve`, `deploy` scripts

### Section F — Homepage: 7/7 (10/10 each)
- F1: 10/10 — `TypewriterCode/index.js` uses `setTimeout` at 40ms to increment `charIndex`, cycles 3 examples with 3s pause, CSS blink cursor
- F2: 10/10 — `HomepageCodeShowcase/index.js` uses `isFading` state with 100ms delay; `styles.module.css` has `.codeContent` transition 200ms
- F3: 10/10 — `HomepageInstall/index.js:25` `navigator.clipboard.writeText()`, 2s copied state, Check/Copy icon toggle
- F4: 10/10 — `HomepageFeatures/styles.module.css:65,71` breakpoints at 768px (2-col) and 480px (1-col)
- F5: 10/10 — `HomepageStats/index.js` renders 4 stats with `.divider` between items
- F6: 10/10 — `HomepageInstall/index.js` has 3 tabs (Cargo/Binary/VS Code) each with copy button
- F7: 10/10 — `index.js` imports all 5 components; build completes without errors

## Total: 57/57 dimensions at 10/10

## Verification Commands Run
- `npm run build`: SUCCESS, 0 errors, 0 warnings
- Element count: 58 across all source files (no "59 element" matches)
- Next steps coverage: 18/18 pages
- No placeholder text: 0 matches for TODO/Lorem/YOUR_USERNAME/coming soon
- Dark mode completeness: all 13 color variables present in dark block
- Search palette: results grouped by page title, close animation with 100ms reverse

## Status: READY FOR PUBLIC LAUNCH
