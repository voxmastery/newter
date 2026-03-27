---
title: Imports
description: Import components, themes, and variables from other Newt files.
keywords: [newt, imports, modules, files, reuse]
sidebar_position: 9
---

# Imports

Imports let you split your Newt code across multiple files and reuse components, themes, and variables.

## Syntax

Use the `import` keyword followed by a string path:

```newt
import "theme.newt";
import "components.newt";
```

Import statements go at the top of the file, before any other declarations.

## Relative paths

Paths are resolved relative to the importing file. Given this file structure:

```
project/
  app.newt
  components/
    cards.newt
    buttons.newt
  themes/
    dark.newt
```

You would import like this:

```newt
// app.newt
import "components/cards.newt";
import "components/buttons.newt";
import "themes/dark.newt";
```

## What gets imported

When you import a file, all of its top-level definitions become available in the importing file:

- **Components** -- any `component` declarations
- **Themes** -- any `theme` declarations (you still need `use theme` to activate one)
- **Variables** -- any `let` declarations

### Example: importing components

```newt
// components.newt
component Avatar(initials) {
    box(fill: #e0e7ff, radius: 999, width: 40, height: 40)(
        center()(
            text { content: initials, fontSize: 14, fontWeight: "700" }
        )
    )
}

component UserRow(name, role) {
    row(gap: 12, align: "center", padding: 8)(
        Avatar("GS")
        column(gap: 2)(
            text { content: name, fontSize: 14, fontWeight: "600" }
            text { content: role, fontSize: 12 }
        )
    )
}
```

```newt
// app.newt
import "components.newt";

screen Main {
    column(gap: 8, padding: 24)(
        UserRow("Ganesh", "Developer")
        UserRow("Alex", "Designer")
    )
}
```

### Example: importing a theme

```newt
// theme.newt
theme Brand {
    let primary = #7c3aed;
    let bg = #f9fafb;
    let text_color = #111827;
    let border = #e5e7eb;
}
```

```newt
// app.newt
import "theme.newt";
use theme Brand;

screen Main {
    column(fill: bg, padding: 32)(
        text("Styled with imports", fill: primary, fontSize: 20)
    )
}
```

Importing the theme file makes the `Brand` theme available, but you still need `use theme Brand` to activate it and expose its variables.

### Example: importing variables

```newt
// config.newt
let maxWidth = 960;
let defaultPadding = 24;
let defaultRadius = 8;
```

```newt
// app.newt
import "config.newt";

screen Main {
    container(padding: defaultPadding)(
        card(radius: defaultRadius, fill: #ffffff, stroke: #e5e7eb, padding: 16)(
            text("Config imported", fontSize: 16)
        )
    )
}
```

## Circular import detection

The compiler detects circular imports and reports an error. If `a.newt` imports `b.newt` and `b.newt` imports `a.newt`, the compiler will refuse to compile and show a clear error message indicating the cycle.

To resolve circular imports, extract the shared definitions into a third file that both can import:

```
// Before (circular):
// a.newt imports b.newt
// b.newt imports a.newt

// After (resolved):
// shared.newt has the common definitions
// a.newt imports shared.newt
// b.newt imports shared.newt
```

## Transitive imports

Imports are not transitive. If `a.newt` imports `b.newt`, and `b.newt` imports `c.newt`, the definitions from `c.newt` are not automatically available in `a.newt`. You need to import `c.newt` explicitly if you want to use its definitions.

```newt
// If you need definitions from both files:
import "b.newt";
import "c.newt";
```

## Best practices

- Keep one theme per file for easy swapping.
- Group related components into a single file (e.g., `cards.newt`, `navigation.newt`).
- Use a `config.newt` for shared constants like spacing, colors, and sizing values.
- Place import statements at the very top of the file for readability.

## Next steps

- [Themes](/docs/language/themes) — define shared themes and import them across files.
- [Components](/docs/language/components) — import reusable components from other files.
- [CLI Reference](/docs/compiler/cli) — compile multi-file projects with the CLI.
