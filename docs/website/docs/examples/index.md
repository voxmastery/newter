---
title: Examples
description: Complete Newt example programs with explanations, including a counter, dashboard, contact form, and landing page.
keywords: [newt, examples, counter, dashboard, form, landing page]
sidebar_position: 1
---

# Examples

Four complete programs that demonstrate different aspects of Newt. Each one is self-contained -- copy it into a `.newt` file and run it with `newter-compiler serve`.

## Counter

A simple counter with increment, decrement, and reset buttons.

```newt
state count = 0;

screen Counter {
    column(gap: 24, padding: 48, fill: #f9fafb)(
        text("Counter", fontSize: 32, fontWeight: "700")
        text("Current value: {count}", fontSize: 18)
        row(gap: 12)(
            button("+ Add", fill: #2563eb, radius: 8, onClick: { count = count + 1 })
            button("- Remove", fill: #6b7280, radius: 8, onClick: { count = count - 1 })
            button("Reset", fill: #ef4444, radius: 8, onClick: { count = 0 })
        )
    )
}
```

:::tip What this demonstrates
- **State**: the `state count = 0` declaration creates a reactive variable.
- **onClick handlers**: each button modifies the `count` state with a different expression.
- **String interpolation**: `"Current value: {count}"` embeds the live value in the text.
- **Layout**: `column` and `row` arrange elements vertically and horizontally with `gap` spacing.
:::

## Dashboard

A full dashboard layout with reusable stat cards, a header with navigation, and a project list built with a for loop.

```newt
let accent = #2563eb;
let bg = #f9fafb;

component StatCard(label, value, color) {
    card(fill: #ffffff, stroke: #e5e7eb, radius: 12, padding: 20)(
        column(gap: 8)(
            text { content: label, fontSize: 12 }
            text { content: value, fontSize: 28, fontWeight: "700" }
        )
    )
}

screen Dashboard {
    column(fill: bg)(
        header(fill: #ffffff, stroke: #e5e7eb, padding: 16)(
            row(gap: 12)(
                text("Dashboard", fontSize: 20, fontWeight: "700")
                spacer()
                button("Settings", stroke: #e5e7eb, radius: 8)
            )
        )
        container(padding: 32)(
            column(gap: 24)(
                row(gap: 16)(
                    StatCard("Total Users", "12,405", accent)
                    StatCard("Revenue", "$48,200", #10b981)
                    StatCard("Growth", "+12.5%", #f59e0b)
                    StatCard("Active Now", "1,893", #8b5cf6)
                )
                for i in range(4) {
                    card(fill: #ffffff, stroke: #e5e7eb, radius: 12, padding: 20)(
                        row(gap: 12)(
                            text("Project {i}", fontSize: 14, fontWeight: "600")
                            spacer()
                            badge { content: "Active", fill: #dcfce7 }
                        )
                    )
                }
            )
        )
    )
}
```

:::tip What this demonstrates
- **Components**: `StatCard` is defined once and used four times with different arguments.
- **let variables**: `accent` and `bg` store colors used throughout the screen.
- **for loops**: `for i in range(4)` generates four project cards dynamically.
- **spacer**: pushes elements apart within a row (the "Settings" button to the right, the badge to the right).
- **Sections**: `header` and `container` provide page structure.
- **Block style props**: `text { content: label, fontSize: 12 }` uses the block syntax inside the component.
:::

## Contact Form

A centered form with labeled inputs and a submit button.

```newt
state submitted = false;
state name = "";

screen ContactForm {
    center(fill: #f9fafb)(
        card(fill: #ffffff, stroke: #e5e7eb, radius: 16, padding: 32)(
            column(gap: 20)(
                text("Contact Us", fontSize: 24, fontWeight: "700")
                column(gap: 8)(
                    text("Name", fontSize: 14, fontWeight: "500")
                    input(stroke: #d1d5db, radius: 8, padding: 12)
                )
                column(gap: 8)(
                    text("Email", fontSize: 14, fontWeight: "500")
                    input(stroke: #d1d5db, radius: 8, padding: 12)
                )
                column(gap: 8)(
                    text("Message", fontSize: 14, fontWeight: "500")
                    input(stroke: #d1d5db, radius: 8, padding: 12)
                )
                button("Send Message", fill: #2563eb, radius: 8, fontSize: 16)
            )
        )
    )
}
```

:::tip What this demonstrates
- **center**: centers the card both horizontally and vertically within the viewport.
- **card**: creates a bordered, rounded container for the form.
- **input elements**: text fields with stroke and radius for a clean form appearance.
- **Nested columns**: an outer column for form sections, inner columns for label + input pairs.
- **State**: `submitted` and `name` are declared for future interactivity (form submission handling).
:::

## Landing Page

A marketing-style landing page with a branded header, hero section, call-to-action buttons, and a footer.

```newt
let brand = #2563eb;

screen Landing {
    column()(
        header(fill: brand, padding: 20)(
            row()(
                text("Newt UI", fontSize: 20, fontWeight: "700")
                spacer()
                button("Get Started", fill: #ffffff, radius: 8)
            )
        )
        container(padding: 80)(
            center()(
                column(gap: 24)(
                    text("Design UIs faster", fontSize: 48, fontWeight: "800")
                    text("A compact language that compiles to canvas, HTML, or JSON", fontSize: 20)
                    row(gap: 12)(
                        button("Try it now", fill: brand, radius: 8, fontSize: 16)
                        button("View docs", stroke: brand, radius: 8, fontSize: 16)
                    )
                )
            )
        )
        footer(fill: #1f2937, padding: 32)(
            center()(
                text("Built with Newt", fontSize: 14)
            )
        )
    )
}
```

:::tip What this demonstrates
- **Page structure**: `header`, `container`, and `footer` create a complete page layout.
- **let variable**: `brand` stores the primary color used across multiple elements.
- **spacer**: pushes the "Get Started" button to the right side of the header.
- **center**: centers the hero content within the container.
- **Button variants**: a filled primary button (`fill: brand`) and an outlined secondary button (`stroke: brand`) side by side.
- **Typography scale**: the headline uses `fontSize: 48` and `fontWeight: "800"` for visual hierarchy, with smaller text below.
:::

## Running the examples

Save any example to a file and run it:

```bash
newter-compiler serve counter.newt
```

Or build it to HTML:

```bash
newter-compiler build dashboard.newt --html -o dashboard.html
```

The example files are also available in the [examples directory](https://github.com/voxmastery/newter/tree/main/examples) of the Newt repository.

## Next steps

- [Getting Started](/docs/getting-started) — install Newt and set up your environment.
- [Elements](/docs/language/elements) — the full reference of built-in UI elements.
- [State](/docs/language/state) — learn how reactive state powers interactive examples.
