---
title: String Interpolation
description: Embed variables and expressions inside strings in Newt using curly brace syntax.
keywords: [newt, string interpolation, template strings, expressions]
sidebar_position: 8
---

# String Interpolation

Newt strings support interpolation using curly braces. Any variable or expression inside `{...}` is evaluated and its result is inserted into the string.

## Basic variable interpolation

Reference a variable by name inside curly braces:

```newt
let name = "Newt";
state count = 0;

screen Main {
    column(gap: 12, padding: 24)(
        text("Hello, {name}!")
        text("Count is {count}")
    )
}
```

When rendered, these produce "Hello, Newt!" and "Count is 0".

## Expressions inside braces

You can put any expression inside the braces, not just variable names:

```newt
state count = 0;

screen Main {
    column(gap: 12, padding: 24)(
        text("Count: {count}")
        text("Double: {count * 2}")
        text("Next: {count + 1}")
        button("Increment", fill: #2563eb, radius: 8, onClick: { count = count + 1 })
    )
}
```

Arithmetic operators (`+`, `-`, `*`, `/`, `%`) all work inside interpolation braces. The expression is evaluated fresh each time the string is rendered, so changes to state variables are reflected immediately.

## Multiple interpolations in one string

A single string can contain multiple interpolated expressions:

```newt
state wins = 7;
state losses = 3;

screen Scoreboard {
    column(gap: 12, padding: 24)(
        text("Record: {wins}W - {losses}L")
        text("Total games: {wins + losses}")
        text("Win rate: {wins}%")
    )
}
```

## Interpolation with let variables

Both `let` and `state` variables work in string interpolation:

```newt
let appName = "TaskFlow";
let version = "1.0";
state tasks = 5;

screen Main {
    column(gap: 12, padding: 24)(
        text("{appName} v{version}")
        text("You have {tasks} tasks remaining")
    )
}
```

## Escaping braces

To include a literal curly brace in a string, escape it with a backslash:

```newt
screen Main {
    column(padding: 24)(
        text("Use \{name\} for interpolation")
    )
}
```

This renders as the literal text "Use &#123;name&#125; for interpolation" without attempting to evaluate `name` as a variable.

## Interpolation in component arguments

String interpolation works in arguments passed to components:

```newt
component Greeting(message) {
    card(fill: #ffffff, stroke: #e5e7eb, radius: 8, padding: 16)(
        text { content: message, fontSize: 18 }
    )
}

let user = "Ganesh";
state unread = 3;

screen Main {
    column(gap: 12, padding: 24)(
        Greeting("Welcome, {user}!")
        Greeting("You have {unread} unread messages")
    )
}
```

## Where interpolation works

String interpolation is supported in any string context:

- The `content` prop of `text` elements (including the shorthand first argument)
- String arguments passed to components
- The `placeholder` prop on input elements
- The `ariaLabel` prop

It does not apply to non-string props like `fill`, `width`, or `onClick`.

## Next steps

- [State](/docs/language/state) — create dynamic values to interpolate into strings.
- [Control Flow](/docs/language/control-flow) — combine string interpolation with conditionals.
- [Props](/docs/language/props) — understand which props accept interpolated strings.
