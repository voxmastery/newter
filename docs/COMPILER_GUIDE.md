# Newt Compiler — Deep Dive Guide

A visual and conceptual guide to how the Newt compiler works, from source code to pixels (or HTML).

---

## Production contract

All consumers (CLI, serve, Canvas IDE, LSP, wgpu app) use the same behavior:

1. **Single entry:** `compile(source, path, screen_name?) -> Result<(Program, LayoutNode), NewtError>` in the compiler library. It (1) parses, (2) resolves imports when `path` is `Some`, (3) builds `EvalContext`, (4) resolves the requested screen, (5) runs `layout_tree` on the screen body. Use this instead of calling `parse`, `resolve_imports`, and `layout_tree` separately.

2. **Imports:** When a file path is provided, `resolve_imports` is run. Without a path (e.g. unsaved buffer), imports are not resolved and import errors will not be reported until the document is saved with a path. The LSP uses the document URI’s file path so import and circular-import errors appear as diagnostics.

3. **Viewport:** The layout tree from `compile()` is computed with viewport `(0, 0, 960, 640)` (constants `DEFAULT_VIEWPORT_W`, `DEFAULT_VIEWPORT_H`). The Canvas IDE and HTML export use this size unless overridden. The wgpu app may recompute layout with the actual window size after compile.

4. **Layout JSON (IDE / clients):** The payload sent to the Canvas IDE (and from `/api/layout`, `/api/compile`) has this shape:
   - `type`: `"layout"` (or `"error"` with `message` on failure).
   - `screens`: array of screen names (strings).
   - `screen`: name of the screen that was laid out.
   - `viewport`: `{ "w": 960, "h": 640 }`.
   - `root`: the root `LayoutNode` (see [Key Data Structures](#7-key-data-structures)) with `kind`, `rect`, `fill`, `stroke`, `radius`, `text`, `font_size`, `children`, etc.

```mermaid
flowchart LR
  subgraph entry
    C[compile]
  end
  subgraph pipeline
    P[parse]
    R[resolve_imports]
    E[EvalContext]
    LT[layout_tree]
  end
  subgraph consumers
    CLI[CLI]
    Serve[serve]
    LSP[LSP]
    App[wgpu App]
  end
  C --> P --> R --> E --> LT
  CLI --> C
  Serve --> C
  LSP --> C
  App --> C
```

### Layout JSON schema (IDE and clients)

The payload sent to the Canvas IDE and from `/api/layout` and `/api/compile` has this shape:

**Root object:**

| Field     | Type   | Description |
|----------|--------|-------------|
| `type`   | string | `"layout"` on success; `"error"` with `message` on failure |
| `screens`| string[] | List of screen names in the program |
| `screen` | string | Name of the screen that was laid out |
| `viewport` | `{ w: number, h: number }` | Viewport size (default 960×640) |
| `root`   | LayoutNode | Root of the layout tree |

**LayoutNode:**

| Field | Type | Description |
|-------|------|-------------|
| `kind` | string | One of: Box, Text, Row, Column, Grid, Stack, Center, Spacer, Image, Button, Input, Modal |
| `rect` | `{ x, y, w, h }` | Bounds (numbers) |
| `fill` | `[r,g,b,a]` or null | Fill color |
| `stroke` | `[r,g,b,a]` or null | Stroke color |
| `stroke_width` | number or null | Border width |
| `radius` | number | Corner radius |
| `text` | string or null | Text content |
| `font_size` | number | Font size |
| `font_weight` | string or null | Font weight |
| `shadow` | number or null | Shadow blur/offset |
| `transition_ms` | number or null | Transition duration (ms) |
| `role` | string or null | ARIA role |
| `aria_label` | string or null | ARIA label |
| `focus_order` | number or null | Tab order |
| `onClick` | string or null | Event hint (data-on-click) |
| `href` | string or null | Link hint (data-href) |
| `name` | string or null | Input name |
| `aspect_ratio` | number or null | Aspect ratio |
| `children` | LayoutNode[] | Child nodes |

Optional **patch** format for incremental updates (e.g. replace/insert/delete node at path) can be added in a later revision so clients can update the canvas without full redraw.

---

## Table of Contents

1. [The Big Picture](#1-the-big-picture)
2. [Stage 1: Lexer (Source → Tokens)](#2-stage-1-lexer-source--tokens)
3. [Stage 2: Parser (Tokens → AST)](#3-stage-2-parser-tokens--ast)
4. [Stage 3: EvalContext (AST → Runtime Context)](#4-stage-3-evalcontext-ast--runtime-context)
5. [Stage 4: Layout (AST + Context → Layout Tree)](#5-stage-4-layout-ast--context--layout-tree)
6. [Stage 5: Render (Layout Tree → GPU or HTML)](#6-stage-5-render-layout-tree--gpu-or-html)
7. [Key Data Structures](#7-key-data-structures)
8. [Example: End-to-End Walkthrough](#8-example-end-to-end-walkthrough)
9. [How to View the Flowcharts](#9-how-to-view-the-flowcharts)

---

## 1. The Big Picture

The compiler is a **linear pipeline**: each stage consumes the output of the previous one. There are two final outputs: **wgpu window** (live canvas) or **HTML file** (static export).

```mermaid
flowchart LR
    subgraph input
        A[.newt source]
    end

    subgraph pipeline
        B[Lexer]
        C[Parser]
        D[EvalContext]
        E[Layout]
    end

    subgraph output
        F[wgpu window]
        G[HTML file]
    end

    A --> B
    B -->|"Token stream"| C
    C -->|"AST (Program)"| D
    D -->|"variables + components"| E
    E -->|"LayoutNode tree"| F
    E -->|"LayoutNode tree"| G
```

**Detailed pipeline (what each stage does):**

```mermaid
flowchart TB
    subgraph s1["1. Lexer"]
        L1["Read source char by char"]
        L2["Skip whitespace & // comments"]
        L3["Emit: Number, String, Ident, HexColor"]
        L4["Emit: keywords (let, screen, box...)"]
        L5["Emit: delimiters { } ( ) , : ; ="]
        L6["Emit: operators + - * / == != < > && ||"]
        L1 --> L2 --> L3 --> L4 --> L5 --> L6
    end

    subgraph s2["2. Parser"]
        P1["Tokenize once (Lexer.next_token loop)"]
        P2["parse() → Program: list of items"]
        P3["Each item: Variable | Component | Screen"]
        P4["parse_expr() with precedence: or → and → == → < → + → * → unary → primary"]
        P5["primary: literal, ident, call, element, block, if, for"]
        P1 --> P2 --> P3 --> P4 --> P5
    end

    subgraph s3["3. EvalContext"]
        E1["From Program: collect all ComponentDecls"]
        E2["From Program: evaluate each VariableDecl in order"]
        E3["Insert name → Value into context.variables"]
        E1 --> E2 --> E3
    end

    subgraph s4["4. Layout"]
        Y1["Find first Screen in Program"]
        Y2["layout_tree(ctx, screen.body, viewport Rect)"]
        Y3["For each Element: get props (padding, gap, fill...)"]
        Y4["Row: split width by children + gap"]
        Y5["Column: split height by children + gap"]
        Y6["Stack/Center/Box: give same rect to all children"]
        Y1 --> Y2 --> Y3 --> Y4 --> Y5 --> Y6
    end

    subgraph s5["5. Render"]
        R1["collect_rects(root) → Vec<DrawRect>"]
        R2["Each DrawRect → 6 vertices (2 triangles)"]
        R3["Vertices in NDC -1..1"]
        R4["wgpu: clear, draw triangle list"]
        R5["OR layout_to_html() → string → file"]
        R1 --> R2 --> R3 --> R4
        R1 --> R5
    end

    s1 --> s2 --> s3 --> s4 --> s5
```

---

## 2. Stage 1: Lexer (Source → Tokens)

The lexer reads the source string **once**, left to right, and produces a **stream of tokens**. Each token has a **kind** and a **span** (start/end byte, line, column).

### Lexer decision flow

```mermaid
flowchart TD
    START([next_token]) --> SKIP[Skip whitespace & // comments]
    SKIP --> PEEK{Peek char}
    PEEK -->|None| EOF[Emit Eof]
    PEEK -->|"#"| HEX[read_hex_color → HexColor]
    PEEK -->|'"'| STR[read_string → String]
    PEEK -->|digit| NUM[read_number → Number]
    PEEK -->|letter or _| ID[read_ident_or_keyword]
    PEEK -->|"-" + ">"| ARR[Emit Arrow]
    PEEK -->|"=="| EQ2[Emit EqEq]
    PEEK -->|"!="| NE[Emit NotEq]
    PEEK -->|"<="| LE[Emit Le]
    PEEK -->|">="| GE[Emit Ge]
    PEEK -->|"&&"| AND[Emit And]
    PEEK -->|"||"| OR[Emit Or]
    PEEK -->|single char| SINGLE[Emit { } ( ) , . ; : = + - * / % < > !]
    PEEK -->|other| ERR[Error: unexpected char]

    ID --> KW{Reserved?}
    KW -->|let, screen, component, if, else, for, in| KTOK[Emit keyword]
    KW -->|box, text, row, column...| KTOK
    KW -->|width, fill, padding...| IDENT[Emit Ident so let padding = 24 works]
    KW -->|other| IDENT
```

### Token categories (summary)

| Category | Examples |
|----------|----------|
| **Literals** | `Number(24)`, `String("hi")`, `HexColor(r,g,b,a)`, `True`, `False` |
| **Keywords** | `Let`, `Screen`, `Component`, `If`, `Else`, `For`, `In` |
| **Element names** | `Box`, `Text`, `Row`, `Column`, `Stack`, `Center`, `Spacer`, `Image`, `Button`, `Input` |
| **Prop names** | In lexer these are emitted as **Ident** (e.g. `padding`, `fill`) so variables like `let padding = 24` work. Parser still recognizes them by string name. |
| **Delimiters** | `LeftBrace` `{`, `RightBrace` `}`, `LeftParen` `(`, `Comma`, `Colon`, `Semicolon`, `Eq` `=` |
| **Operators** | `Plus`, `Minus`, `Star`, `Slash`, `EqEq`, `NotEq`, `Lt`, `Le`, `Gt`, `Ge`, `And`, `Or`, `Not` |

---

## 3. Stage 2: Parser (Tokens → AST)

The parser is **recursive descent**: one function per grammar rule, consuming tokens via `advance()` and `expect()`.

### Top-level: Program

```mermaid
flowchart LR
    subgraph Program
        P[parse]
        P --> V[parse_variable]
        P --> C[parse_component]
        P --> S[parse_screen]
    end
    P -->|"while !Eof"| DECIDE{Current token?}
    DECIDE -->|Let| V
    DECIDE -->|Component| C
    DECIDE -->|Screen| S
    DECIDE -->|other| ERR[Parse error]
    V --> P
    C --> P
    S --> P
```

- **Variable**: `let` ident `=` expr `;`
- **Component**: `component` ident `(` params? `)` `{` expr `}`
- **Screen**: `screen` ident `{` expr `}`

### Expression precedence (low to high)

The parser uses a **precedence ladder**: lower precedence calls higher. So "or" is at the top, "primary" at the bottom.

```mermaid
flowchart TB
    subgraph prec["Precedence (lowest at top)"]
        OR["parse_or()     →  ||"]
        AND["parse_and()   →  &&"]
        EQ["parse_equality()   →  == !="]
        CMP["parse_comparison() →  < <= > >="]
        TERM["parse_term()     →  + -"]
        FACT["parse_factor()   →  * / %"]
        UNARY["parse_unary()   →  ! -"]
        PRI["parse_primary()   →  literals, ident, call, element, block, if, for"]
    end
    OR --> AND --> EQ --> CMP --> TERM --> FACT --> UNARY --> PRI
```

So for `a + b * c`, we get: term calls factor; factor consumes `b * c` first (higher precedence), then term adds `a` and the result.

### Primary expressions

```mermaid
flowchart TD
    PRI([parse_primary]) --> CUR{Current token?}
    CUR -->|Number| LIT[Literal Number]
    CUR -->|String| LIT2[Literal String]
    CUR -->|True/False| LIT3[Literal Bool]
    CUR -->|HexColor| LIT4[Literal Color]
    CUR -->|"{"| BLOCK[Block: stmts then expr]
    CUR -->|if| IF[If: cond, then, else?]
    CUR -->|for| FOR[For: var, iter, body]
    CUR -->|Ident| IDENT{Next?}
    IDENT -->|"("| CALL[Call: callee, args]
    IDENT -->|element keyword| ELT[Element: kind, props, children]
    IDENT -->|other| REF[Ident reference]
    CUR -->|box, text, row...| ELT
```

### Element parsing (props + children)

Elements look like: `box { fill: #fff, padding: 16 } { text { content: "Hi" } }` — optional props block, optional children block.

```mermaid
flowchart TD
    ELT([parse_element_props_and_children]) --> LB1{At "{"?}
    LB1 -->|yes| PROPS[While not "}": parse_prop or parse_expr]
    LB1 -->|no| CHILD
    PROPS --> LB2{At "{"?}
    LB2 -->|yes| CHILD[While not "}": parse_expr = child]
    LB2 -->|no| DONE
    CHILD --> DONE[Return Expr::Element]
```

---

## 4. Stage 3: EvalContext (AST → Runtime Context)

Before layout, we need a **runtime context**: variable names → values, and component names → definitions. Layout and expression evaluation both use this.

```mermaid
flowchart TB
    subgraph input
        PROG[Program]
    end

    subgraph build["EvalContext::from_program(program)"]
        STEP1[1. Create empty context]
        STEP2[2. Insert all ComponentDecls into context.components]
        STEP3[3. For each VariableDecl in order:]
        STEP4["   eval_expr(ctx, decl.value) → Value"]
        STEP5["   context.variables.insert(decl.name, value)"]
    end

    subgraph output
        CTX[EvalContext]
        CTX --> VARS[variables: HashMap name → Value]
        CTX --> COMPS[components: HashMap name → ComponentDecl]
    end

    PROG --> STEP1 --> STEP2 --> STEP3 --> STEP4 --> STEP5 --> CTX
```

**Important:** Variables are evaluated in **program order**. So you can use a variable only after it’s defined (no forward references). Component bodies are **not** evaluated here; they are evaluated when a **Call** is seen during layout or eval.

---

## 5. Stage 4: Layout (AST + Context → Layout Tree)

Layout turns the **screen body expression** (a tree of Elements, Calls, Blocks) into a tree of **LayoutNodes**, each with a **Rect** and visual props (fill, stroke, radius, text, font_size).

### Entry point

```mermaid
flowchart LR
    A[First Screen in Program] --> B[screen.body = Expr]
    B --> C[viewport = Rect 0,0, width, height]
    C --> D[layout_tree ctx body viewport]
    D --> E[LayoutNode tree]
```

### layout_tree dispatch

```mermaid
flowchart TD
    LT([layout_tree ctx expr rect]) --> M{expr variant?}
    M -->|Element| ELEM[Layout element]
    M -->|Call| CALL[Resolve component, bind args, layout_tree body]
    M -->|Block| BLOCK[Layout last Expr stmt in block]
    M -->|other| DEFAULT[Empty Box LayoutNode]
```

### Layout element (the core)

For `Expr::Element { kind, props, children }`:

1. **Resolve props** (using `ctx`): padding, gap, fill, stroke, radius, fontSize, content (text).
2. **Compute inner rect**: shrink `rect` by padding on all sides.
3. **Dispatch by kind** to compute child rects and recurse.

```mermaid
flowchart TB
    ELEM([Element: kind, props, children]) --> PROPS[get_prop_number/color/string]
    PROPS --> INNER[inner = rect - padding]
    INNER --> KIND{kind?}

    KIND -->|Row| ROW[Split inner width by child count + gap]
    ROW --> R1[Each child: rect x, y, child_w, inner.h]
    R1 --> RECUR1[layout_tree each child]

    KIND -->|Column| COL[Split inner height by child count + gap]
    COL --> C1[Each child: rect inner.x, y, inner.w, child_h]
    C1 --> RECUR2[layout_tree each child]

    KIND -->|Stack / Center / Box / Button / Input| SAME[Each child gets same inner rect]
    SAME --> RECUR3[layout_tree each child]

    KIND -->|Text / Spacer / Image| NONE[children = empty]
```

So:

- **Row**: horizontal strip; equal width per child; gap between.
- **Column**: vertical strip; equal height per child; gap between.
- **Stack / Center / Box / Button / Input**: all children get the **same** inner rect (no sizing by content yet).
- **Text / Spacer / Image**: no child nodes; text/content stored for HTML or future text renderer.

---

## 6. Stage 5: Render (Layout Tree → GPU or HTML)

### GPU path (wgpu)

```mermaid
flowchart TB
    ROOT[LayoutNode root] --> COLLECT[collect_rects root]
    COLLECT --> WALK[Walk tree depth-first]
    WALK --> FILTER{Node kind?}
    FILTER -->|Box, Row, Column, Stack, Center, Button, Input| PUSH[Push DrawRect]
    FILTER -->|Text with fill| PUSH
    FILTER -->|Text no fill / Spacer / Image| SKIP[Skip]
    PUSH --> MORE[Recurse to children]
    SKIP --> MORE
    MORE --> NEXT[Next node]

    PUSH --> RECT[DrawRect: x, y, w, h, radius, fill, stroke]
    RECT --> VERT[rect_to_vertices: 6 vertices per rect]
    VERT --> NDC[Positions in NDC -1..1, Y flipped]
    NDC --> BUF[write_buffer vertex_buffer]
    BUF --> PASS[RenderPass: clear, set pipeline, draw]
```

- **collect_rects**: produces `Vec<DrawRect>`. Stroke and radius are stored but the current **rect.wgsl** shader does not use them (draws only solid quads).
- **rect_to_vertices**: each rect → 6 vertices (2 triangles); color from fill.
- **render()**: get current surface texture, clear, draw triangle list, present.

### HTML path

```mermaid
flowchart LR
    ROOT[LayoutNode root] --> EMIT[emit_node recursive]
    EMIT --> DIV["<div style='left,top,width,height; fill; stroke; radius; font-size'>"]
    DIV --> TEXT[Text content if any]
    TEXT --> KIDS[emit_node each child]
    KIDS --> CLOSE[</div>]
```

`layout_to_html` produces a single HTML file with one root div; each node is a positioned div with inline styles.

---

## 7. Key Data Structures

### AST (excerpt)

```mermaid
classDiagram
    class Program {
        +items: Vec~ProgramItem~
    }
    class ProgramItem {
        <<enum>>
        Variable(VariableDecl)
        Component(ComponentDecl)
        Screen(ScreenDecl)
    }
    class Expr {
        <<enum>>
        Literal(Literal)
        Ident(String, Span)
        Binary(left, op, right)
        Unary(op, inner)
        Call(callee, args)
        Element(kind, props, children)
        Block(stmts)
        If(cond, then, else?)
        For(var, iter, body)
    }
    class ElementKind {
        <<enum>>
        Box, Text, Row, Column, Stack, Center, Spacer, Image, Button, Input
    }
    class Prop {
        +name: PropName
        +value: PropValue
    }
    Program --> ProgramItem
    Expr --> ElementKind
    Expr --> Prop
```

### Layout

```mermaid
classDiagram
    class Rect {
        +x: f32
        +y: f32
        +w: f32
        +h: f32
    }
    class LayoutNode {
        +kind: LayoutKind
        +rect: Rect
        +fill: Option~(u8,u8,u8,u8)~
        +stroke: Option~(u8,u8,u8,u8)~
        +radius: f32
        +text: Option~String~
        +font_size: f32
        +children: Vec~LayoutNode~
    }
    class LayoutKind {
        <<enum>>
        Box, Text, Row, Column, Stack, Center, Spacer, Image, Button, Input
    }
    LayoutNode --> Rect
    LayoutNode --> LayoutNode
```

### Value & EvalContext

```mermaid
classDiagram
    class Value {
        <<enum>>
        Number(f64)
        String(String)
        Bool(bool)
        Color(r,g,b,a)
    }
    class EvalContext {
        +variables: HashMap~String, Value~
        +components: HashMap~String, ComponentDecl~
    }
    EvalContext --> Value
```

---

## 8. Example: End-to-End Walkthrough

Source (simplified):

```newt
let padding = 24;
screen Main {
  column { gap: 16, padding: padding } {
    box { fill: #ffffff, radius: 8 } {
      text { content: "Hello", fontSize: 24 }
    }
  }
}
```

### Step 1 — Lexer (conceptual token stream)

```
Let, Ident("padding"), Eq, Number(24), Semicolon,
Screen, Ident("Main"), LeftBrace,
Column, LeftBrace, Ident("gap"), Colon, Number(16), Comma, Ident("padding"), Colon, Ident("padding"), RightBrace, LeftBrace,
Box, LeftBrace, Ident("fill"), Colon, HexColor(255,255,255), Comma, Ident("radius"), Colon, Number(8), RightBrace, LeftBrace,
Text, LeftBrace, Ident("content"), Colon, String("Hello"), Comma, Ident("fontSize"), Colon, Number(24), RightBrace,
RightBrace, RightBrace, RightBrace, RightBrace,
Eof
```

### Step 2 — Parser

- **Program.items**: [ Variable(padding = 24), Screen(Main, body) ].
- **body** = `Expr::Element { kind: Column, props: [gap: 16, padding: ident "padding"], children: [ box_element ] }`.
- **box_element** = `Expr::Element { kind: Box, props: [fill: #fff, radius: 8], children: [ text_element ] }`.
- **text_element** = `Expr::Element { kind: Text, props: [content: "Hello", fontSize: 24], children: [] }`.

### Step 3 — EvalContext

- **variables**: `padding → Value::Number(24)`.
- **components**: (none).

### Step 4 — Layout (viewport 960×640)

1. **layout_tree(column, viewport)**  
   - padding = 24 (eval of ident "padding" → 24), gap = 16.  
   - inner = (24, 24, 912, 592).  
   - One child → one slot: (24, 24, 912, 592).

2. **layout_tree(box, (24,24,912,592))**  
   - fill = #ffffff, radius = 8.  
   - inner = same (no padding on box in this example).  
   - One child → (24, 24, 912, 592).

3. **layout_tree(text, (24,24,912,592))**  
   - text = "Hello", font_size = 24.  
   - children = [].

Result: root Column → one child Box → one child Text; all share the same rect; Box has fill/radius, Text has content/fontSize.

### Step 5 — Render

- **collect_rects**: one DrawRect for the Box (fill white, radius 8; stroke/radius not yet used in shader). Text node might add a rect only if it had a fill; here it doesn’t draw a rect in wgpu.
- Vertices for that rect are uploaded; render pass clears and draws.  
- **HTML**: root div 960×640; inside it a div for the column, then box, then text div with "Hello" and font-size.

---

## 9. How to View the Flowcharts

The diagrams in this file are written in **Mermaid**. You can view them in:

- **GitHub / GitLab**: paste this file in a repo; they render Mermaid in `.md` files.
- **VS Code**: install the “Markdown Preview Mermaid Support” extension, then open this file and use the markdown preview (e.g. `Ctrl+Shift+V`).
- **Notion**: create a code block, set language to `Mermaid`, and paste a single Mermaid diagram (one code block per diagram). Notion will render it.
- **Online**: copy a Mermaid code block into [mermaid.live](https://mermaid.live) to edit and export as PNG/SVG.

If you want this as a **single Notion page**: create a page, then paste each section and each Mermaid block (as a Mermaid code block) into that page for a one-place compiler guide.
