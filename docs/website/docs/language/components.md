---
title: Components
description: Define and use reusable UI components in Newt with parameters and nesting.
keywords: [newt, components, reusable, parameters, composition]
sidebar_position: 4
---

# Components

Components let you define a reusable piece of UI once and use it anywhere. They accept parameters, can contain any elements, and can be nested inside each other.

## Defining a component

Use the `component` keyword followed by a name and a parameter list:

```newt
component StatCard(label, value) {
    card(fill: #ffffff, stroke: #e5e7eb, radius: 12, padding: 20)(
        column(gap: 8)(
            text { content: label, fontSize: 12 }
            text { content: value, fontSize: 28, fontWeight: "700" }
        )
    )
}
```

The component name must start with an uppercase letter. Parameters are listed inside parentheses, separated by commas. Inside the component body, parameters are used by name -- they behave like local variables.

## Using a component

Call a component the same way you call a built-in element, passing arguments positionally:

```newt
component StatCard(label, value) {
    card(fill: #ffffff, stroke: #e5e7eb, radius: 12, padding: 20)(
        column(gap: 8)(
            text { content: label, fontSize: 12 }
            text { content: value, fontSize: 28, fontWeight: "700" }
        )
    )
}

screen Main {
    row(gap: 16, padding: 24)(
        StatCard("Users", "12,405")
        StatCard("Revenue", "$48,200")
        StatCard("Growth", "+12.5%")
    )
}
```

Arguments are matched to parameters in order: the first argument maps to `label`, the second to `value`.

## Complete example

Here is a full program with a component definition and usage:

```newt
component StatCard(label, value) {
    card(fill: #ffffff, stroke: #e5e7eb, radius: 12, padding: 20)(
        column(gap: 8)(
            text { content: label, fontSize: 12 }
            text { content: value, fontSize: 28, fontWeight: "700" }
        )
    )
}

screen Main {
    row(gap: 16)(
        StatCard("Users", "12,405")
        StatCard("Revenue", "$48,200")
    )
}
```

## Nesting components

Components can use other components in their body:

```newt
component IconLabel(iconName, label) {
    row(gap: 8, align: "center")(
        icon(iconName)
        text { content: label, fontSize: 14 }
    )
}

component NavItem(iconName, label) {
    box(padding: 12, radius: 8)(
        IconLabel(iconName, label)
    )
}

screen Sidebar {
    sidebar(fill: #1f2937, padding: 16)(
        column(gap: 4)(
            NavItem("home", "Dashboard")
            NavItem("users", "Team")
            NavItem("settings", "Settings")
        )
    )
}
```

This works because `NavItem` is defined after `IconLabel`, and the compiler resolves all component definitions before rendering. The order of definitions does not matter -- you can reference a component that is declared later in the file.

## Components with styling parameters

You can pass colors, sizes, and other values as parameters to make components flexible:

```newt
component Badge(label, color) {
    box(fill: color, radius: 12, padding: 8)(
        text { content: label, fontSize: 12, fontWeight: "600" }
    )
}

screen Main {
    row(gap: 8)(
        Badge("Active", #dcfce7)
        Badge("Pending", #fef3c7)
        Badge("Closed", #fee2e2)
    )
}
```

## Components from other files

You can define components in separate files and import them:

```newt
// components.newt
component Avatar(name) {
    box(fill: #e0e7ff, radius: 999, width: 40, height: 40)(
        center()(
            text { content: name, fontSize: 14, fontWeight: "700" }
        )
    )
}
```

```newt
// app.newt
import "components.newt";

screen Main {
    row(gap: 12)(
        Avatar("GS")
        Avatar("JD")
    )
}
```

See the [Imports](./imports.md) page for full details on the import system.

## Guidelines

- Name components with PascalCase (e.g., `StatCard`, `NavItem`, `UserAvatar`).
- Keep components focused on a single responsibility.
- Use parameters for any value that changes between instances.
- Extract repeated UI patterns into components to keep screens readable.

## Next steps

- [State](/docs/language/state) — add interactivity to your components with reactive state.
- [Themes](/docs/language/themes) — apply consistent styling across components with themes.
- [Examples](/docs/examples) — see components used in complete programs.
