# Newt Language Reference for LLMs

Newt is a call-style UI DSL that compiles to layout trees. Files use `.newt` extension.

## Syntax Basics

```
screen Main {
  let pad = 16;
  column(padding: pad, gap: 12) {
    text("Hello")
    button("Click me")
  }
}
```

**Two equivalent styles:**
- Call: `row(gap: 12)(text("Hi") button("OK"))`
- Block: `row { gap: 12 } { text { content: "Hi" } button { content: "OK" } }`

**Content shorthand:** First string arg becomes `content` prop: `text("Hello")` = `text { content: "Hello" }`.

## Literals

`42`, `3.14` (f64) | `"hello"` (string, supports `\n \t \" \\`) | `#ff0000`, `#00ff0080` (hex color RGBA) | `true`, `false` | `[1, 2, 3]` (array)

## Variables

```
let bg = #1a1a2e;
let size = 18;
```

## State

```
state count = 0;
```

## String Interpolation

```
text("Count: {count}")
```

Expressions inside `{}` within strings are evaluated and coerced to string.

## All Elements (72)

**Sections:** header, footer, container, sidebar, section
**Layout:** row, column, stack, center, box, widget, card, grid
**Nav:** accordion, bento, breadcrumb, hamburger, kebab, meatballs, doner, tabs, pagination, linkList, nav
**Input:** button, input, password, search, checkbox, radio, dropdown, combobox, multiselect, datePicker, picker, slider, stepper, toggle, form, select, textarea
**Feedback:** modal, confirmDialog, toast, notification, alert, messageBox, tooltip, loader, progressBar, badge
**Display:** text, icon, tag, comment, feed, carousel, chart, image, spacer
**v0.2:** table, avatar, skeleton, drawer, popover, separator, timeline, rating, fileUpload, colorPicker, treeView, commandPalette, splitter

## All Prop Names

**Layout:** width, height, padding, gap, grow, shrink, align, justify, direction
**Constraints:** minWidth, maxWidth, minHeight, maxHeight, aspectRatio
**Style:** fill, stroke, strokeWidth, radius, fontSize, fontWeight, shadow, transition
**Grid:** columns, rows
**Content:** content, src, name, href
**Events:** onClick
**A11y:** role, ariaLabel, focusOrder
**Custom:** any identifier works as `Ident(name)` prop

## Components

```
component Card(title, color) {
  box(fill: color, padding: 16, radius: 8) {
    text(title, fontSize: 20, fontWeight: 700)
  }
}
// Usage:
Card("Settings", #2d2d44)
```

## Themes

```
theme Dark {
  let bg = #1a1a2e;
  let fg = #eeeeff;
  let accent = #e94560;
}
use theme Dark;
```

Theme variables merge into scope when `use theme` is encountered.

## Control Flow

```
if showDetails {
  text("Details here")
} else {
  text("Hidden")
}

for i in range(5) {
  text("Item {i}")
}
```

`range(n)` returns `[0, 1, ..., n-1]`.

## Imports

```
import "components.newt";
import "themes/dark.newt";
```

Paths are relative to the current file. Circular imports error.

## Operators

Arithmetic: `+ - * / %` | Comparison: `== != < <= > >=` | Logical: `&& || !` | Unary: `- !`

## Multi-Screen

```
screen Home { ... }
screen About { ... }
```

## Comments

```
// single-line only
```

## Key Gotcha

Inside a component, when passing a variable as the first argument to an element, use **block syntax** to avoid parse ambiguity:

```
// WRONG - parser may treat `label` as a component call
text(label, fontSize: 14)

// CORRECT - use block syntax
text { content: label, fontSize: 14 }
```

## Example 1: Counter

```
screen Counter {
  state count = 0;
  column(padding: 24, gap: 16, align: "center") {
    text("Count: {count}", fontSize: 32)
    row(gap: 12) {
      button("- 1", onClick: "decrement")
      button("+ 1", onClick: "increment")
    }
  }
}
```

## Example 2: Card Component

```
component InfoCard(title, description, color) {
  card(fill: color, radius: 12, padding: 20) {
    column(gap: 8) {
      text { content: title, fontSize: 18, fontWeight: 700 }
      text { content: description, fontSize: 14 }
    }
  }
}

screen Cards {
  column(padding: 16, gap: 12) {
    InfoCard("Speed", "Blazing fast rendering", #0a3d62)
    InfoCard("Safety", "Type-safe UI language", #1e3799)
  }
}
```

## Example 3: Themed Layout

```
theme Ocean {
  let bg = #0a1628;
  let surface = #142d4c;
  let text_color = #e8f1f8;
  let accent = #3dc1d3;
}
use theme Ocean;

component NavItem(label) {
  button { content: label, fill: surface, radius: 6, padding: 8 }
}

screen Dashboard {
  column(fill: bg, height: 800) {
    header(fill: surface, padding: 16) {
      row(gap: 16, align: "center") {
        text("Dashboard", fontSize: 24, fontWeight: 700)
        spacer()
        NavItem("Home")
        NavItem("Settings")
      }
    }
    row(padding: 16, gap: 16, grow: 1) {
      sidebar(fill: surface, width: 200, padding: 12) {
        column(gap: 8) {
          for i in range(4) {
            text("Menu {i}", fontSize: 14)
          }
        }
      }
      column(gap: 16, grow: 1) {
        row(gap: 16) {
          box(fill: accent, radius: 8, padding: 16, grow: 1) {
            text("Active Users: 1,204", fontSize: 16)
          }
          box(fill: surface, radius: 8, padding: 16, grow: 1) {
            text("Revenue: $45.2k", fontSize: 16)
          }
        }
        card(fill: surface, radius: 8, padding: 16, grow: 1) {
          text("Chart area", fontSize: 14)
        }
      }
    }
  }
}
```
