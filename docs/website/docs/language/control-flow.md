---
title: Control Flow
description: Conditional rendering with if/else and iteration with for loops in Newt.
keywords: [newt, if, else, for, loop, conditional, control flow]
sidebar_position: 6
---

# Control Flow

Newt supports two control flow constructs: `if/else` for conditional rendering and `for` loops for repetition.

## If / else

Show or hide parts of the UI based on a condition.

```newt
state loggedIn = false;

screen App {
    column(gap: 16, padding: 32)(
        if loggedIn {
            column(gap: 12)(
                text("Welcome back!", fontSize: 24, fontWeight: "700")
                button("Log Out", fill: #ef4444, radius: 8, onClick: { loggedIn = false })
            )
        } else {
            column(gap: 12)(
                text("Please sign in", fontSize: 24, fontWeight: "700")
                button("Log In", fill: #2563eb, radius: 8, onClick: { loggedIn = true })
            )
        }
    )
}
```

The condition can be any expression that evaluates to a boolean:

```newt
state count = 0;

screen Main {
    column(gap: 16, padding: 32)(
        text("Count: {count}", fontSize: 24)
        button("Add", fill: #2563eb, radius: 8, onClick: { count = count + 1 })

        if count > 10 {
            alert(fill: #fef3c7, stroke: #f59e0b, radius: 8, padding: 12)(
                text("Count is getting high!", fontSize: 14)
            )
        }

        if count == 0 {
            text("Click the button to start counting.", fontSize: 14)
        }
    )
}
```

### If without else

The `else` branch is optional. If omitted, nothing is rendered when the condition is false.

### Comparison operators in conditions

You can use any comparison or logical operator:

- `==` and `!=` for equality checks
- `<`, `>`, `<=`, `>=` for numeric comparisons
- `&&` for logical AND
- `||` for logical OR

```newt
state age = 25;
state hasTicket = true;

screen Gate {
    column(gap: 16, padding: 32)(
        if age >= 18 && hasTicket {
            text("Access granted", fontSize: 20, fontWeight: "700")
        } else {
            text("Access denied", fontSize: 20, fontWeight: "700")
        }
    )
}
```

## For loops

Repeat elements a set number of times using `for` with `range()`.

```newt
screen List {
    column(gap: 8, padding: 24)(
        for i in range(5) {
            card(fill: #ffffff, stroke: #e5e7eb, radius: 8, padding: 12)(
                text("Item {i}")
            )
        }
    )
}
```

`range(5)` produces the values `0, 1, 2, 3, 4`. The loop variable `i` is available inside the loop body and can be used in string interpolation or expressions.

### Using the loop variable

The loop variable works in expressions, props, and string interpolation:

```newt
screen Gradient {
    column(gap: 4, padding: 24)(
        for i in range(8) {
            box(height: 40, radius: 4, fill: #2563eb)(
                text("Row {i}", fontSize: 14)
            )
        }
    )
}
```

### Nested loops

Loops can be nested to create grid-like structures:

```newt
screen Grid {
    column(gap: 8, padding: 24)(
        for r in range(3) {
            row(gap: 8)(
                for col in range(4) {
                    box(width: 60, height: 60, fill: #e0e7ff, radius: 8)(
                        center()(
                            text("{col}", fontSize: 12)
                        )
                    )
                }
            )
        }
    )
}
```

### Practical example: a task list

Combine `for` loops with components and state for dynamic UIs:

```newt
state taskCount = 3;

component TaskRow(index) {
    card(fill: #ffffff, stroke: #e5e7eb, radius: 8, padding: 12)(
        row(gap: 12)(
            checkbox()
            text("Task {index}", fontSize: 14)
            spacer()
            badge { content: "Pending", fill: #fef3c7 }
        )
    )
}

screen Tasks {
    column(gap: 12, padding: 24)(
        row(gap: 12)(
            text("My Tasks", fontSize: 24, fontWeight: "700")
            spacer()
            button("Add Task", fill: #2563eb, radius: 8, onClick: { taskCount = taskCount + 1 })
        )
        for i in range(taskCount) {
            TaskRow(i)
        }
    )
}
```

## Next steps

- [Components](/docs/language/components) — combine control flow with reusable components.
- [State](/docs/language/state) — drive conditional rendering with reactive state.
- [Examples](/docs/examples) — see control flow in working programs.
