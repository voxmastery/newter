---
title: Themes
description: Define and apply themes in Newt for consistent styling across your UI.
keywords: [newt, themes, colors, styling, dark mode, light mode]
sidebar_position: 7
---

# Themes

Themes let you define a named set of variables for consistent styling. You declare a theme once, activate it with `use theme`, and then reference its variables throughout your screens and components.

## Defining a theme

Use the `theme` keyword followed by a name and a block of `let` declarations:

```newt
theme Dark {
    let bg = #1a1a2e;
    let text_color = #e0e0e0;
    let accent = #7c3aed;
}
```

A theme is simply a named collection of variables. The variables can hold any literal value -- colors, numbers, or strings.

## Activating a theme

Use the `use theme` statement to activate a theme. Its variables become available in the rest of the program:

```newt
theme Dark {
    let bg = #1a1a2e;
    let text_color = #e0e0e0;
    let accent = #7c3aed;
}

use theme Dark;

screen Main {
    column(fill: bg, padding: 32)(
        text("Dark Mode", fontSize: 24, fill: text_color)
        button("Action", fill: accent, radius: 8)
    )
}
```

After `use theme Dark`, the variables `bg`, `text_color`, and `accent` are available everywhere -- in screens, components, and expressions.

## Multiple themes

You can define multiple themes and switch between them. Only one theme can be active at a time:

```newt
theme Light {
    let bg = #ffffff;
    let surface = #f9fafb;
    let text_color = #111827;
    let accent = #2563eb;
    let border = #e5e7eb;
}

theme Dark {
    let bg = #111827;
    let surface = #1f2937;
    let text_color = #f9fafb;
    let accent = #818cf8;
    let border = #374151;
}

use theme Light;

screen Main {
    column(fill: bg, padding: 32)(
        card(fill: surface, stroke: border, radius: 12, padding: 20)(
            column(gap: 12)(
                text("Themed Card", fontSize: 20, fontWeight: "700", fill: text_color)
                text("This card respects the active theme.", fontSize: 14, fill: text_color)
                button("Primary Action", fill: accent, radius: 8)
            )
        )
    )
}
```

To switch to the dark theme, change `use theme Light` to `use theme Dark`. All the variable references update automatically.

## Themes with components

Theme variables work inside components just like they do in screens:

```newt
theme Brand {
    let primary = #7c3aed;
    let bg = #faf5ff;
    let surface = #ffffff;
    let border = #e9d5ff;
    let text_color = #1e1b4b;
}

use theme Brand;

component NavButton(label) {
    button { content: label, fill: primary, radius: 8, fontSize: 14 }
}

component InfoCard(title, description) {
    card(fill: surface, stroke: border, radius: 12, padding: 20)(
        column(gap: 8)(
            text { content: title, fontSize: 18, fontWeight: "700", fill: text_color }
            text { content: description, fontSize: 14, fill: text_color }
        )
    )
}

screen Main {
    column(fill: bg, padding: 32, gap: 24)(
        row(gap: 12)(
            NavButton("Home")
            NavButton("About")
            NavButton("Contact")
        )
        InfoCard("Welcome", "This UI is styled with the Brand theme.")
    )
}
```

## Importing themes

Themes can live in their own files and be imported:

```newt
// brand-theme.newt
theme Brand {
    let primary = #7c3aed;
    let bg = #faf5ff;
    let text_color = #1e1b4b;
}
```

```newt
// app.newt
import "brand-theme.newt";
use theme Brand;

screen Main {
    column(fill: bg)(
        text("Styled with imported theme", fill: text_color)
    )
}
```

See [Imports](./imports.md) for full details on the import system.

## Theme variable naming

Theme variables follow the same rules as `let` variables. Use descriptive names that indicate purpose rather than specific colors:

| Recommended      | Avoid        |
|-----------------|--------------|
| `bg`            | `white`      |
| `text_color`    | `black`      |
| `accent`        | `blue`       |
| `border`        | `gray`       |
| `surface`       | `lightGray`  |

This makes it easier to create alternative themes where the same variable maps to a different color.

## Next steps

- [Imports](/docs/language/imports) — share themes across files with imports.
- [Components](/docs/language/components) — use themed components for consistent design.
- [CSS Variables](/docs/compiler/html-export) — see how theme variables map to CSS custom properties in HTML export.
