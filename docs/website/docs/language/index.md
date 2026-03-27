---
title: Language Overview
description: A complete overview of Newt syntax, including elements, props, literals, operators, and program structure.
keywords: [newt, syntax, language, overview, elements, props]
sidebar_position: 1
---

# Language Overview

Newt is a declarative UI language. You describe what the interface looks like, and the compiler handles layout and rendering. This page covers the syntax fundamentals.

## Two syntax styles

Every element in Newt can be written in either **call style** or **block style**. They are interchangeable -- use whichever reads better for the situation.

### Call style

Props go in the first set of parentheses. Children go in the second.

```newt
row(gap: 12)(
    text("Hello")
    text("World")
)
```

### Block style

Props and children each get their own curly-brace block.

```newt
row { gap: 12 } {
    text { content: "Hello" }
    text { content: "World" }
}
```

You can mix styles within the same program. Call style is more concise for simple elements; block style can be easier to read when an element has many props.

## Literals

Newt supports four literal types:

| Type    | Examples                        | Notes                                  |
|---------|---------------------------------|----------------------------------------|
| Number  | `16`, `3.14`, `0`               | Integer or floating-point              |
| String  | `"Hello"`, `"multi word"`       | Double-quoted, supports interpolation  |
| Color   | `#2563eb`, `#ff0000`, `#f9fafb` | Six-digit hex, prefixed with `#`       |
| Boolean | `true`, `false`                 | Used in conditions and toggle state    |

## Operators

Newt supports arithmetic, comparison, and logical operators:

| Category    | Operators              |
|-------------|------------------------|
| Arithmetic  | `+` `-` `*` `/` `%`   |
| Comparison  | `==` `!=` `<` `>` `<=` `>=` |
| Logical     | `&&` `||`              |

Operators work in expressions, conditions, and string interpolation:

```newt
state count = 0;

screen Main {
    text("Double: {count * 2}")
}
```

## Comments

Line comments start with `//`. Everything after `//` on that line is ignored.

```newt
// This is a comment
screen Main {
    text("Hello") // inline comment
}
```

## Program structure

A Newt program is a collection of top-level items. These can appear in any order:

### Variables

Immutable bindings declared with `let`:

```newt
let accent = #2563eb;
let title = "My App";
```

### State

Reactive variables declared with `state`. Changes to state trigger re-renders:

```newt
state count = 0;
state darkMode = false;
```

### Components

Reusable UI fragments with parameters:

```newt
component Badge(label) {
    box(fill: #e0e7ff, radius: 12, padding: 8)(
        text { content: label, fontSize: 12, fontWeight: "600" }
    )
}
```

### Screens

Named entry points for rendering. A program can have multiple screens:

```newt
screen Home {
    text("Home page")
}

screen Settings {
    text("Settings page")
}
```

Use the `--screen` flag to choose which screen to render.

### Themes

Named collections of variables for consistent styling:

```newt
theme Light {
    let bg = #ffffff;
    let fg = #111827;
}
```

### Imports

Pull in definitions from other `.newt` files:

```newt
import "components.newt";
```

## Putting it together

Here is a small but complete program that uses several of these features:

```newt
let accent = #2563eb;
state clicks = 0;

component Counter(label) {
    card(fill: #ffffff, stroke: #e5e7eb, radius: 12, padding: 20)(
        column(gap: 8)(
            text { content: label, fontSize: 14 }
            text("{clicks}", fontSize: 28, fontWeight: "700")
            button("Click me", fill: accent, radius: 8, onClick: { clicks = clicks + 1 })
        )
    )
}

screen Main {
    center(fill: #f9fafb)(
        Counter("Total clicks")
    )
}
```

## Next steps

- [Elements](./elements.md) -- the full list of 58 built-in elements.
- [Props](./props.md) -- every prop with its type, default, and description.
- [Components](./components.md) -- how to define and use reusable components.
- [State](./state.md) -- reactive state and event handlers.
