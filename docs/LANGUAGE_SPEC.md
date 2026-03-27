# Newt UI Language — Full Specification

Newt is a call-style UI DSL for designing screens, layouts, and UI elements. This document specifies the complete syntax, semantics, and element roster.

---

## 1. Syntax Overview

### 1.1 General Patterns

- **Screen**: `screen(Main) { ... }` or `screen Name { ... }`
- **Elements**: `elementName(props?)(children?)` or `elementName { props } { children }`
- **Content shorthand**: `text("Hello")`, `button("OK")` — first string becomes `content`
- **Variables**: `let name = value;`
- **Components**: `component Name(params?) { body }`
- ** Themes**: `theme Name { let var = value; }` and `use theme Name;`
- **Imports**: `import "path.newt";`

### 1.2 Two Syntax Styles (Both Supported)

| Style | Example |
|-------|---------|
| Call style | `row(gap:12)(box() text("Hi"))` |
| Block style | `row { gap: 12 } { box {} text { content: "Hi" } }` |

---

## 2. Tokens (Lexer)

### 2.1 Literals

| Token | Example | Notes |
|-------|---------|-------|
| Number | `42`, `3.14` | f64 |
| String | `"hello"` | Supports `\n`, `\t`, `\"`, `\\` |
| HexColor | `#ff0000`, `#00ff0080` | 6 or 8 hex digits (RGBA) |
| True/False | `true`, `false` | Boolean literals |
| Ident | `myVar`, `padding` | Alphanumeric + underscore |

### 2.2 Keywords (Language)

`let`, `screen`, `component`, `theme`, `use`, `import`, `if`, `else`, `for`, `in`

### 2.3 Element Tokens (UI Primitives)

**Sections**: `header`, `footer`, `container`, `sidebar`, `section`  
**Layout**: `row`, `column`, `stack`, `center`, `box`, `widget`, `card`, `grid`  
**Nav**: `accordion`, `bento`, `breadcrumb`, `hamburger`, `kebab`, `meatballs`, `doner`, `tabs`, `pagination`, `linkList`, `nav`  
**Input**: `button`, `input`, `password`, `search`, `checkbox`, `radio`, `dropdown`, `combobox`, `multiselect`, `datePicker`, `picker`, `slider`, `stepper`, `toggle`, `form`  
**Feedback**: `modal`, `confirmDialog`, `toast`, `notification`, `alert`, `messageBox`, `tooltip`, `loader`, `progressBar`, `badge`  
**Display**: `text`, `icon`, `tag`, `comment`, `feed`, `carousel`, `chart`, `image`, `spacer`

### 2.4 Prop Names (Parser Recognizes)

Layout: `width`, `height`, `padding`, `gap`, `grow`, `shrink`, `align`, `justify`, `direction`  
Style: `fill`, `stroke`, `strokeWidth`, `radius`, `fontSize`, `fontWeight`, `shadow`, `transition`  
Content: `content`  
Constraints: `minWidth`, `maxWidth`, `minHeight`, `maxHeight`, `aspectRatio`  
Grid: `columns`, `rows`  
A11y: `role`, `ariaLabel`, `focusOrder`  
Other: `src`, `onClick`, `href`, `name`

*Note: Lexer emits prop keywords as `Ident` so `let padding = 24;` works.*

### 2.5 Delimiters and Operators

`{ } ( ) [ ] , . ; : = ->`  
`+ - * / % == != < <= > >= && || !`

### 2.6 Comments

`//` line comments (rest of line skipped)

### 2.7 Token Categories (for Syntax Highlighting)

| Category | Purpose |
|----------|---------|
| Keyword | `let`, `screen`, `if`, etc. |
| String | `"..."` |
| Number | `42`, `3.14` |
| Ident | Variable/component names |
| Operator | `+`, `==`, etc. |
| Punctuation | `{`, `(`, `,`, etc. |
| Comment | `// ...` |
| Eof | End of file |

---

## 3. AST (Abstract Syntax Tree)

### 3.1 Program

```rust
pub struct Program {
    pub items: Vec<ProgramItem>,
}
```

### 3.2 ProgramItem

| Variant | Syntax |
|---------|--------|
| Variable | `let x = 42;` |
| Component | `component Card(title) { ... }` |
| Screen | `screen Main { ... }` |
| Theme | `theme Dark { let bg = #1a1a1a; }` |
| Import | `import "lib.newt";` |
| UseTheme | `use theme Dark;` |

### 3.3 Expressions (Expr)

| Variant | Example |
|---------|---------|
| Literal | `42`, `"hi"`, `true`, `#ff0000` |
| Ident | `x`, `padding` |
| Binary | `a + b`, `x == 5`, `a && b` |
| Unary | `!flag`, `-n` |
| Call | `Card("Hi", #ff0000)`, `range(5)` |
| Element | `box { fill: #fff } { text { content: "Hi" } }` |
| Block | `{ let x = 1; x + 2 }` |
| If | `if cond { then } else { else }` |
| For | `for i in range(5) { ... }` |

### 3.4 Literal

| Variant | Example |
|---------|---------|
| Number | `42`, `3.14` |
| String | `"hello"` |
| Bool | `true`, `false` |
| Color | From HexColor token |
| Array | `[1, 2, 3]` |

### 3.5 Element

```rust
Expr::Element {
    kind: ElementKind,  // Box, Text, Row, Column, ...
    props: Vec<Prop>,
    children: Vec<Expr>,
    span: Span,
}
```

### 3.6 Prop

```rust
struct Prop {
    name: PropName,   // Ident or keyword (Width, Fill, etc.)
    value: PropValue, // Number, String, Color, or Expr
    span: Span,
}
```

### 3.7 Built-in Functions

| Function | Args | Returns | Usage |
|----------|------|---------|-------|
| range(n) | n: number | Array [0..n-1] | `for i in range(5)` |

---

## 4. Layout Semantics

### 4.1 ElementKind → LayoutKind Mapping

| ElementKind | LayoutKind | Behavior |
|-------------|------------|----------|
| Header, Footer, Container, Sidebar, Section, Widget | Column | Vertical stack |
| Accordion, Bento, Tabs, Nav, Form, Feed, Carousel, ... | Column | Vertical stack |
| Modal | Modal | (Modal overlay) |
| Card, Box, Button, Input, Checkbox, ... | Box | Same rect to all children |
| Text | Text | No children |
| Row | Row | Horizontal split by child count + gap |
| Column | Column | Vertical split |
| Grid | Grid | Grid with `columns`/`rows` spec |
| Stack, Center | Stack/Center | Same rect to children |
| Spacer | Spacer | No children, no visual in wgpu |
| Image | Image | No children |

### 4.2 Layout Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| padding | number | 0 | Inner inset |
| gap | number | 0 | Space between children |
| width, height | number | flexible | Fixed size |
| minWidth, maxWidth | number | 0, ∞ | Visibility (viewport-based) |
| minHeight, maxHeight | number | 0, ∞ | Visibility |
| grow, shrink | number | - | Flex-like (partial support) |
| fill | color | - | Background |
| stroke | color | - | Border (stored, not rendered in wgpu yet) |
| strokeWidth | number | 1 | Border width |
| radius | number | 0 | Corner radius (stored, not in shader yet) |
| fontSize | number | 16 | Text size |
| content | string | - | Text content |
| aspectRatio | number | - | Constrains rect |
| columns, rows | string | "1fr" | Grid tracks |

### 4.3 Visibility

Elements with `minWidth`/`maxWidth`/`minHeight`/`maxHeight` are hidden when viewport does not satisfy constraints. Hidden elements produce empty LayoutNodes.

---

## 5. Evaluation (EvalContext)

### 5.1 Variable Resolution

- Variables evaluated in program order.
- No forward references.
- Theme variables merged when `use theme X` is seen.

### 5.2 Component Calls

- Params bound by name/position.
- New EvalContext with params, then `layout_tree(ctx, comp.body, rect)`.

### 5.3 Expression Evaluation

- Binary ops: `+ - * / % == != < <= > >= && ||`
- Unary: `! -`
- Block: evaluates stmts in order; `Let` in block does **not** mutate outer context (block-scoped vars not persisted).
- For: `range(n)` → `[0,1,...,n-1]`; body evaluated per iteration; result is array of body results.

---

## 6. Control Flow

### 6.1 If/Else

- Condition must evaluate to bool.
- Branch expressions can be any Expr (including elements).

### 6.2 For Loops

- `for var in iter { body }`
- `iter` typically `range(n)`.
- Body can contain elements; layout_tree expands For into multiple child LayoutNodes.

---

## 7. File Structure

- One file can have multiple screens, components, themes.
- `import "path.newt"` loads and merges items; circular imports error.
- Import path relative to current file's directory.

---

## 8. Multi-Screen

- `get_screen(program, None)` → first screen.
- `get_screen(program, Some("About"))` → screen by name.
- `compile(source, path, Some("About"))` compiles the About screen.
- Layout JSON includes `screens` array for IDE screen selector.

---

## 9. Content Shorthand

- `text("Hello")` ≡ `text { content: "Hello" }`
- `button("OK")` ≡ `button { content: "OK" }`
- First positional string in props becomes `content` when applicable.

---

## 10. Adding New Elements (Compiler Changes)

Per `.cursor/rules/newt-ui-language.mdc`:

1. **Lexer** (`lexer.rs`): Add `TokenKind` variant and match in `read_ident_or_keyword`.
2. **AST** (`ast.rs`): Add `ElementKind` variant and `from_token_kind` mapping.
3. **Layout** (`layout.rs`): Map `ElementKind` → `LayoutKind` in `layout_kind_from_element`.
