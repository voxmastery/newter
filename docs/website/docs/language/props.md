---
title: Props
description: Complete reference for all Newt element props, including layout, style, content, constraints, and interaction properties.
keywords: [newt, props, properties, layout, style, accessibility]
sidebar_position: 3
---

# Props

Props control the appearance and behavior of elements. They are passed as key-value pairs inside parentheses (call style) or curly braces (block style).

```newt
// Call style
button("Save", fill: #2563eb, radius: 8, fontSize: 16)

// Block style
button { content: "Save", fill: #2563eb, radius: 8, fontSize: 16 }
```

## Layout props

Control how an element is sized and how it arranges its children.

| Prop        | Type   | Default    | Description                                                     |
|-------------|--------|------------|-----------------------------------------------------------------|
| `width`     | number | flexible   | Fixed width in pixels. Omit to let the layout engine decide.    |
| `height`    | number | flexible   | Fixed height in pixels. Omit to let the layout engine decide.   |
| `padding`   | number | `0`        | Inner spacing on all four sides in pixels.                      |
| `gap`       | number | `0`        | Space between children in pixels (for row, column, grid).       |
| `grow`      | number | --         | Flex grow factor. Higher values claim more available space.      |
| `shrink`    | number | --         | Flex shrink factor. Controls how an element shrinks when space is tight. |
| `align`     | string | --         | Cross-axis alignment: `"start"`, `"center"`, `"end"`, `"stretch"`. |
| `justify`   | string | --         | Main-axis alignment: `"start"`, `"center"`, `"end"`, `"between"`, `"around"`. |
| `direction` | string | --         | Override default direction: `"row"` or `"column"`.              |

```newt
screen LayoutPropsDemo {
    row(gap: 16, padding: 24, align: "center")(
        box(width: 100, height: 100, fill: #e0e0e0, radius: 8)(text("Fixed"))
        box(grow: 1, height: 100, fill: #dbeafe, radius: 8)(text("Grows"))
        box(width: 100, height: 100, fill: #e0e0e0, radius: 8)(text("Fixed"))
    )
}
```

## Style props

Control visual appearance.

| Prop          | Type   | Default  | Description                                                    |
|---------------|--------|----------|----------------------------------------------------------------|
| `fill`        | color  | none     | Background color. Accepts hex colors like `#2563eb`.           |
| `stroke`      | color  | none     | Border color.                                                  |
| `strokeWidth` | number | `1`      | Border thickness in pixels.                                    |
| `radius`      | number | `0`      | Corner radius in pixels. Set to create rounded corners.        |
| `fontSize`    | number | `16`     | Text size in pixels.                                           |
| `fontWeight`  | string | `"400"`  | Text weight. Common values: `"400"` (normal), `"600"` (semi-bold), `"700"` (bold), `"800"` (extra-bold). |
| `shadow`      | number | none     | Box shadow spread in pixels.                                   |
| `transition`  | number | none     | Transition duration in milliseconds for animated prop changes. |

```newt
screen StylePropsDemo {
    column(gap: 16, padding: 24)(
        card(fill: #ffffff, stroke: #e5e7eb, strokeWidth: 2, radius: 16, shadow: 4, padding: 20)(
            text("Styled card", fontSize: 18, fontWeight: "700")
        )
        button("Hover me", fill: #2563eb, radius: 8, transition: 200)
    )
}
```

## Content prop

| Prop      | Type   | Default | Description                                           |
|-----------|--------|---------|-------------------------------------------------------|
| `content` | string | none    | The text content of an element. Can also be passed as the first positional argument. |

These two forms are equivalent:

```newt
text("Hello, world!")
text { content: "Hello, world!" }
```

## Constraint props

Limit an element's dimensions.

| Prop          | Type   | Default | Description                          |
|---------------|--------|---------|--------------------------------------|
| `minWidth`    | number | `0`     | Minimum width in pixels.             |
| `maxWidth`    | number | none    | Maximum width in pixels.             |
| `minHeight`   | number | `0`     | Minimum height in pixels.            |
| `maxHeight`   | number | none    | Maximum height in pixels.            |
| `aspectRatio` | number | none    | Width-to-height ratio (e.g., `1.5` for 3:2). |

```newt
screen ConstraintDemo {
    column(gap: 16, padding: 24)(
        box(minWidth: 200, maxWidth: 600, fill: #dbeafe, radius: 8, padding: 16)(
            text("Width is clamped between 200px and 600px")
        )
        image(src: "photo.png", aspectRatio: 1.778, radius: 8)
    )
}
```

## Grid props

Control grid layout when using the `grid` element.

| Prop      | Type   | Default | Description                                                      |
|-----------|--------|---------|------------------------------------------------------------------|
| `columns` | string | `"1fr"` | Column template using CSS grid syntax (e.g., `"1fr 1fr"`, `"200px 1fr"`). |
| `rows`    | string | `"1fr"` | Row template using CSS grid syntax.                              |

```newt
screen GridDemo {
    grid(columns: "1fr 2fr 1fr", rows: "auto auto", gap: 12, padding: 24)(
        box(fill: #fee2e2, radius: 8, padding: 12)(text("Narrow"))
        box(fill: #dbeafe, radius: 8, padding: 12)(text("Wide"))
        box(fill: #dcfce7, radius: 8, padding: 12)(text("Narrow"))
        box(fill: #fef3c7, radius: 8, padding: 12)(text("Row 2"))
        box(fill: #e0e7ff, radius: 8, padding: 12)(text("Row 2"))
        box(fill: #fce7f3, radius: 8, padding: 12)(text("Row 2"))
    )
}
```

## Accessibility props

Provide semantic information for assistive technologies.

| Prop         | Type   | Default | Description                                         |
|--------------|--------|---------|-----------------------------------------------------|
| `role`       | string | none    | ARIA role (e.g., `"navigation"`, `"main"`, `"alert"`). |
| `ariaLabel`  | string | none    | Accessible label for screen readers.                |
| `focusOrder` | number | none    | Tab order for keyboard navigation.                  |

```newt
screen AccessibilityDemo {
    column(gap: 16)(
        nav(role: "navigation", ariaLabel: "Main navigation")(
            row(gap: 16)(
                text("Home")
                text("About")
            )
        )
        button("Submit", fill: #2563eb, radius: 8, ariaLabel: "Submit the contact form", focusOrder: 1)
    )
}
```

## Interaction props

Handle user actions and link to resources.

| Prop          | Type       | Default | Description                                           |
|---------------|------------|---------|-------------------------------------------------------|
| `onClick`     | expression | none    | Code to run when the element is clicked.              |
| `href`        | string     | none    | URL to navigate to when clicked.                      |
| `src`         | string     | none    | Source URL for images.                                |
| `name`        | string     | none    | Identifier for form inputs.                           |
| `placeholder` | string     | none    | Hint text shown in empty input fields.                |

```newt
state expanded = false;

screen InteractionDemo {
    column(gap: 16, padding: 24)(
        button("Toggle details", fill: #2563eb, radius: 8, onClick: { expanded = !expanded })
        input(name: "email", placeholder: "you@example.com", stroke: #d1d5db, radius: 8, padding: 12)
        image(src: "https://example.com/logo.png", width: 120)
    )
}
```

## Next steps

- [Elements](/docs/language/elements) — the full list of built-in elements that accept these props.
- [State](/docs/language/state) — make props dynamic with reactive state variables.
- [Components](/docs/language/components) — pass props as parameters to reusable components.
