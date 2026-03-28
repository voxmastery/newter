# Newt improvements to support React-style UIs (typewriter analysis)

This document analyzes the **typewriter** React app’s UI patterns and derives **concrete improvements** for the Newt language and compiler so that, over time, Newt can express similar UIs.

---

## 1. React/typewriter patterns analyzed

### 1.1 Layout and structure

| Pattern | Example (typewriter) | Newt today |
|--------|----------------------|------------|
| **Fixed grid** | `grid-cols-[80px_1fr_340px]` (Toolbar \| Main \| Sidebar) | No grid; only row/column (flex). Sidebar + main done with a single row. |
| **Constrained width** | `max-w-6xl mx-auto`, `max-w-md w-full` | No `maxWidth` on container semantics; only responsive min/max visibility. |
| **Aspect ratio** | `aspect-[4/3]`, `aspect-[4/5.5]` (ProjectCard, NewProjectButton) | No aspect ratio. |
| **Flexible grid** | `grid-cols-[repeat(auto-fill,minmax(280px,1fr))]` | No auto-fill / repeat; can’t express “as many cards as fit”. |
| **Fixed sizes** | `w-12 h-12`, `w-8 h-8`, `w-full` | `width`/`height` in AST/layout not used to size boxes; space comes from parent split. |
| **Vertical text** | `writing-mode: vertical-rl` (Toolbar label) | No text orientation. |
| **Overflow** | `overflow-hidden`, `overflow-y-auto`, `truncate` | No overflow/truncation semantics. |

### 1.2 Styling and design tokens

| Pattern | Example (typewriter) | Newt today |
|--------|----------------------|------------|
| **CSS variables** | `var(--bg-bone)`, `var(--ink-black)`, `var(--border-radius)` | Variables only in Newt (e.g. `let bone = #EBE9E4`); no export as CSS vars. |
| **Opacity variants** | `border-ink/10`, `bg-ink/5`, `text-ink/50` | No alpha/opacity; only solid colors. |
| **Semantic tokens** | `bg-bone`, `bg-acid`, `text-ink`, `border-ink` | Same idea via variables; no built-in “semantic color” role. |
| **Border width** | `border`, `border-2` | Single stroke; no stroke width prop. |
| **Shadow** | `shadow-brutal` (8px 8px 0 #111) | `shadow` (number) in layout/HTML; no offset/color. |
| **Border radius** | `rounded-3xl` (24px), `rounded-full` (9999px) | `radius` (number); no “full” or token. |
| **Font family** | `font-type` (Courier Prime), `font-ui` (Inter) | No font family; only fontSize/fontWeight. |

### 1.3 Components and reuse

| Pattern | Example (typewriter) | Newt today |
|--------|----------------------|------------|
| **Parameterized components** | `<ModeCard card={...} />`, `<StatRow label value />` | Components with params; no “slot” or “children” as first-class. |
| **Composition** | `<MainLayout sidebar={sidebar}><Content /></MainLayout>` | No slot props (e.g. named slots like `sidebar`). |
| **Lists from data** | `MODE_CARDS.map(card => <ModeCard key={...} />)` | `for var in array { ... }` in layout; good. |
| **Conditional layout** | `{filter === 'all' ? ... : ...}`, `{isPublished && <Badge />}` | `if cond { then } else { else }` in layout; good. |

### 1.4 Interactivity and behavior

| Pattern | Example (typewriter) | Newt today |
|--------|----------------------|------------|
| **Navigation** | `navigate('/dashboard')`, `<Link to="..." />` | None; no routes or links. |
| **Click / events** | `onClick`, `onKeyDown`, `onMouseEnter` | None; layout only. |
| **State** | `useState`, `useAppStore()` | None. |
| **Hover/focus styles** | `hover:bg-acid`, `focus:ring-2` | None; no states. |
| **Form binding** | `value={email}`, `onChange` | Input is visual only; no binding. |

### 1.5 Motion and animation

| Pattern | Example (typewriter) | Newt today |
|--------|----------------------|------------|
| **Page transition** | AnimatePresence, route change animation | None. |
| **Stagger** | `staggerChildren`, `delayChildren` | None. |
| **Hover/tap** | `whileHover`, `whileTap`, lift/tilt (CardTransition) | None. |
| **Drill transition** | Card → full screen with shared rect | None. |
| **Transition config** | `duration`, `ease`, spring | `transition` (ms) only; no easing. |

### 1.6 Accessibility and semantics

| Pattern | Example (typewriter) | Newt today |
|--------|----------------------|------------|
| **Role / ARIA** | `role="button"`, `aria-label`, `tabIndex={0}` | `role`, `ariaLabel`, `focusOrder` in AST and HTML. |
| **Keyboard** | Enter/Space on list item | No event model. |

### 1.7 Content and typography

| Pattern | Example (typewriter) | Newt today |
|--------|----------------------|------------|
| **Uppercase / tracking** | `uppercase`, `tracking-widest`, `text-[10px]` | No textTransform, letterSpacing. |
| **Multi-line / truncate** | `truncate`, `whitespace-nowrap`, `line-clamp` | Single text node; no truncation. |
| **Rich text** | Bold/italic spans, inline links | Plain text only. |

---

## 2. Prioritized improvements for Newt

Below are **concrete** improvements so Newt can move toward typewriter-like UIs. Order is by impact and dependency.

---

### 2.1 Layout engine

**Goal:** Support typewriter-like structure (fixed columns, aspect ratio, max width, grid).

1. **Grid layout**
   - Add a **grid** (or **gridRow**) element with props such as `columns: [80, "1fr", 340]` or `columns: "auto 1fr 340px"`.
   - Layout: compute column widths (fixed px or fr) and place children in cells (row-major or explicit row/col).

2. **Fixed width/height on nodes**
   - Use existing **width** / **height** props in layout: when set, give the node that size instead of splitting parent space equally (row/column still divide *remaining* space among flexible children).
   - Support **minWidth** / **maxWidth** / **minHeight** / **maxHeight** for flex-like behavior (already in AST; ensure layout uses them).

3. **Aspect ratio**
   - New prop: **aspectRatio** (e.g. `aspectRatio: 4/3` or `aspectRatio: "4/3"`). Layout: from parent-given width or height, compute the other dimension so the ratio holds.

4. **Max content width**
   - **maxWidth** on container/column with “center in viewport” semantics so “max-w-6xl mx-auto” can be expressed (e.g. container with maxWidth and align center).

5. **Overflow and truncation**
   - **overflow**: "hidden" | "scroll" | "visible" (for HTML/export).
   - **textOverflow**: "ellipsis" | "clip" (for text nodes); requires max width on text (or parent) to be meaningful.

---

### 2.2 Theming and design tokens

**Goal:** Match typewriter’s use of CSS variables and opacity variants.

1. **Opacity / alpha on colors**
   - Allow alpha in literals and variables: e.g. `#11111180`, or `inkAlpha: ink @ 0.5` (expression: color @ number).
   - Emit in HTML as `rgba(...)` or `color-mix` so borders/backgrounds can be ink/10, bone/80, etc.

2. **Semantic color roles**
   - Optional: **theme** could define not only variables but *roles* like `primary`, `surface`, `border`, `text`, and components refer to roles. Then “acid” vs “ink” as primary is a theme choice.

3. **Export design tokens to CSS**
   - When exporting HTML (or a design-token file), emit `:root { --bone: #EBE9E4; --acid: #FDFD66; --ink: #111111; }` from Newt variables/themes so the same tokens can drive CSS.

4. **Stroke width**
   - **strokeWidth**: number (default 1). Use in layout/HTML for border width.

5. **Shadow with offset and color**
   - Extend **shadow** to something like `shadow: [offsetX, offsetY, blur?, color?]` or named `shadow: "brutal"` from theme (e.g. 8 8 0 ink). Emit as box-shadow in HTML.

6. **Radius “full”**
   - Allow **radius: "full"** or a very large number to mean “pill/circle” (e.g. for buttons and avatars).

---

### 2.3 Typography

**Goal:** Support labels and headings like typewriter (uppercase, small caps, tracking).

1. **textTransform**: "none" | "uppercase" | "lowercase" | "capitalize".
2. **letterSpacing**: number or token (e.g. "wide", "wider").
3. **fontFamily**: string or token (e.g. "type" → Courier Prime, "ui" → Inter); require a way to map names to font stacks in export.
4. **lineHeight**: number or "normal".

Store in AST/layout and emit in HTML so headings and labels can match typewriter style.

---

### 2.4 Slots and composition

**Goal:** Express MainLayout(sidebar + main) and reusable shells.

1. **Named slots**
   - Screen or component can declare slots: e.g. `screen Dashboard { slot sidebar; slot main; layout row ( sidebar main ) }`, and at call site: `Dashboard ( sidebar: ( ... ) main: ( ... ) )`. Or: `component MainLayout(sidebar?, main) { row ( toolbar sidebar main ) }` with **named** children: e.g. `MainLayout ( sidebar: column(...) main: column(...) )`.
   - Parser: allow **slotName: expr** among children for a component/screen that declares those slots; layout places them in the right places.

2. **Default slot**
   - One unnamed slot for “main content” so `Card ( "Title" row(...) )` maps to title + body without naming.

This lets Newt describe the same structural composition as typewriter (toolbar + main + sidebar) and reuse layout shells.

---

### 2.5 State and events (longer-term)

**Goal:** Buttons and inputs that “do something” in a host app or in a future Newt runtime.

1. **Declarative events (no implementation in Newt)**
   - Add optional **onClick**, **onChange** (and later **onFocus**, **onBlur**) as **prop names only** (e.g. `button("Save", onClick: "save")`). Meaning: “this element has a click handler named save.” The compiler can emit `data-on-click="save"` or similar in HTML; a host (React, web component) wires the name to a real handler. Newt stays “layout + intent,” not scripting.

2. **Navigation intent**
   - **link** element or **href** on button: `button("Dashboard", href: "/dashboard")`. Export as `<a href="...">` or `data-navigate="/dashboard"` so a router can handle it. No router in Newt; just the target.

3. **Form and input identity**
   - **name** on input (and optionally **type**: "text" | "email" | "password"). Export as `name="..."` and `type="..."` so a host can bind form data. No state in Newt.

These keep Newt declarative while making it possible for a React (or other) host to “plug in” behavior.

---

### 2.6 Motion (longer-term)

**Goal:** Document animation intent for a host or future runtime.

1. **Transition names**
   - **transition**: already ms; add optional **transitionName**: "fade" | "slide" | "drill" so export or runtime can map to Framer Motion variants or CSS.

2. **Stagger**
   - On container: **staggerChildren**: number (ms), **delayChildren**: number (ms). Emit as data attributes or CSS vars for a host to use (e.g. `data-stagger="50"`).

3. **Hover/focus style blocks (design only)**
   - Optional **hover: { ... }** / **focus: { ... }** as a set of prop overrides (e.g. `hover: { shadow: 8 }`). Used in design/preview and possibly exported as CSS or a style map; no behavior in Newt.

---

### 2.7 Responsive and breakpoints

**Goal:** Match typewriter’s use of breakpoints (e.g. `sm:`, `lg:`).

1. **Named breakpoints in theme**
   - e.g. `theme App { let sm = 640; let md = 768; let lg = 1024; }`. Use in **minWidth**/ **maxWidth** as variables so `maxWidth: lg` means “hide or show below 1024.”

2. **Responsive props (list)**
   - Allow a prop to be a list of breakpoint values: e.g. `padding: [16, 24, 32]` with default breakpoints [0, 640, 1024]; layout picks the value for current viewport. Optional; can start with a single value only.

---

### 2.8 HTML and export

**Goal:** Generated HTML/CSS should be enough for a “static” or host-driven version of typewriter-style screens.

1. **Emit CSS variables** from theme/variables (see 2.2).
2. **Emit font-family** when **fontFamily** is set (see 2.3).
3. **Emit box-shadow** from extended shadow (see 2.2).
4. **Emit data-* for events and navigation** (see 2.5) so a small JS layer or React wrapper can attach behavior.
5. **Optional: class names** — e.g. **class**: "card" or **className**: "card" so export adds a class; then a separate CSS file (or design system) can define .card. Lets Newt stay minimal while matching existing CSS.

---

## 3. Summary table

| Area | Improvement | Newt change |
|------|-------------|------------|
| Layout | Grid (columns) | New element + layout logic |
| Layout | Use width/height | Layout engine uses existing props |
| Layout | Aspect ratio | New prop aspectRatio |
| Layout | Max width + center | maxWidth + align semantics |
| Layout | Overflow / truncation | New props, HTML output |
| Tokens | Color with alpha | Color @ number or #RRGGBBAA |
| Tokens | Stroke width, shadow (offset/color) | New/extended props |
| Tokens | Export CSS vars | From theme/variables in HTML |
| Typography | textTransform, letterSpacing, fontFamily | New props, HTML |
| Composition | Named slots | Slots in component/screen, slotName: expr |
| Behavior | onClick/href/name (intent only) | Optional props, data-* in HTML |
| Motion | Stagger, transition name | Optional props, data-* or CSS |
| Responsive | Breakpoint variables | Use theme numbers in minWidth/maxWidth |

---

## 4. Suggested implementation order

1. **Layout:** Use **width**/ **height** in layout; add **aspectRatio**; then **grid** (or fixed columns in row).  
2. **Tokens:** **Opacity** on colors; **strokeWidth**; **shadow** with offset/color; **radius: "full"**; emit CSS vars.  
3. **Typography:** **textTransform**, **letterSpacing**, **fontFamily** (and **lineHeight**).  
4. **Slots:** Named slots for components/screens.  
5. **Export:** data-* for events and navigation; optional class names.  
6. **Motion/breakpoints:** Stagger, transition name, breakpoint variables.

This order gets “layout and look” close to typewriter first, then composition, then behavior and motion so that Newt can eventually describe such UIs and export them for a React (or other) host to bring to life.
