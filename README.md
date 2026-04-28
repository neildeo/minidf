# Mini DataFrame (Rust)

A small, educational, columnar dataframe library implemented in Rust.

This project is designed as a learning exercise in analytical data systems, focusing on clear semantics, strong typing, and test-driven development rather than performance or completeness.

---

## Goals

- Build a minimal but principled dataframe abstraction
- Understand columnar data modelling and analytical operations
- Provide a foundation for future extensions:
  - metrics / semantic layer
  - data quality testing
  - visualisation tooling
- Practise Rust design and test-driven development

---

## Non-Goals (Phase 1)

- High performance
- Full SQL compatibility
- Lazy execution / query optimisation
- Complete file format support (e.g. full Parquet implementation)
- Advanced data types (dates, nested types, categoricals, etc.)

---

## Core Philosophy

### Columnar Design

- Data is stored as **columns**, not rows
- Each column is:
  - typed
  - nullable
  - equal in length to all other columns

A dataframe is an **ordered collection of uniquely named columns**.

---

### Immutability

- All operations return new dataframes
- Original dataframes are never mutated
- Enables predictable, testable behaviour

---

### Expressions

- Computation is described using **expressions**
- Expressions are evaluated eagerly, but remain reusable and composable

Examples (conceptual):

```
col("age") > 30
col("revenue") / col("quantity")
is_null(col("customer_id"))
```

---

### Separation of Concerns

The system is conceptually divided into:

- **Storage layer** — how data is physically represented
- **Expression layer** — how computations are described
- **Operation layer** — how transformations are applied

Operations should not depend on storage internals.

---

### Schema as First-Class

Each dataframe has an explicit schema:

- ordered fields
- unique column names
- defined types
- nullability

---

## Data Model

A dataframe consists of:

- **Schema**
  - ordered fields
  - unique names
  - types
  - nullability

- **Columns**
  - typed vectors of values
  - nullable

- **Row count**
  - all columns must have equal length

---

## Semantics

### Null Handling (SQL-style)

- Null represents **missing or unknown data**
- Arithmetic with null → null
- Comparisons with null → null
- Boolean logic uses three-valued logic:
  - `true AND null → null`
  - `false AND null → false`
  - `true OR null → true`
  - `false OR null → null`

### Filtering

- Only rows where predicate evaluates to **true** are kept
- `false` and `null` rows are dropped

### Aggregation

- `sum`, `mean`, `min`, `max` ignore nulls
- `count(column)` counts non-null values
- `count(*)` counts all rows

### Grouping

- Group-by defines output grain
- No grouping → single-row output
- Null keys form their own group
- No guaranteed output order unless explicitly sorted

### Joins

- Supported (Phase 1):
  - inner join
  - left join
- Join keys must have compatible types
- Null keys do not match
- Duplicate non-key column names must be resolved (e.g. suffix or explicit rename)

### Type System

- Strong typing with limited coercion
- Allowed:
  - numeric widening (e.g. `Int + Float → Float`)
- Disallowed:
  - unrelated implicit coercions (e.g. `String + Int`)

---

## API Design Principles

### Column Names

- Must be unique
- Duplicate names are forbidden
- Operations that would create duplicates must error

### Column Order

- Column order is preserved
- Order is part of dataframe semantics
- Affects display, export, and user expectations

### `with_column`

- Adds a column if it does not exist
- Replaces a column if it already exists

### Row Access

- Rows are not first-class storage
- Row-like access exists for:
  - `head` / `tail`
  - display
  - testing
  - I/O

---

## CSV I/O

### Supported Modes

1. **Explicit schema**
2. **Inferred schema**
3. **Raw (all strings)**

### Schema Inference

- Based on all rows (Phase 1)
- Chooses the least general compatible type
- Type widening order:

```
Bool → Int → Float → String
```

### Parsing Rules

- Missing values → null
- Invalid typed values → error (default)
- No silent coercion of invalid values

---

## Error Handling

- Operations are **fail-fast**
- Errors are structured (not stringly typed)

Example categories:

```
ColumnNotFound
DuplicateColumnName
TypeMismatch
LengthMismatch
InvalidExpression
ParseError
```

- Design should allow future extension to:
  - validation mode
  - multiple error accumulation

---

## Phase 1 Feature Set

### Core

- Dataframe construction from columns
- Schema validation
- Shape and schema inspection
- Head / tail display

### Transformations

- Select columns
- Filter rows (via expressions)
- Add/replace columns
- Sort rows

### Aggregation

- Dataframe-wide aggregation
- Group-by aggregation

### Joins

- Inner join
- Left join

### I/O

- CSV read
- CSV write
- Explicit schema support
- Schema inference

---

## Testing Philosophy

- Test behaviour, not implementation
- Focus on:
  - correctness
  - semantics
  - immutability
- Avoid coupling tests to internal representation

---

## Future Extensions

- Lazy execution (logical plans / DAG)
- Validation layer (error accumulation)
- Metrics / semantic layer
- Data quality checks
- Additional file formats (Parquet)
- Advanced types (dates, categoricals, nested data)

---

## Summary

This project aims to build a **small, principled analytical kernel**:

- columnar
- typed
- immutable
- expression-driven

It prioritises clarity of semantics over performance, serving as a foundation for deeper exploration of modern data systems.