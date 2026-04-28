# Roadmap

This project will be developed in small, test-driven phases. The roadmap is intentionally coarse: detailed implementation decisions should be made within each milestone.

## Phase 0: Repository setup

Set up the Rust crate, GitHub repository, project documentation, and basic development workflow.

Target outcomes:

- Rust library crate exists
- README contains the project spec
- ROADMAP exists
- GitHub repo is created
- Initial tests can run locally
- Basic CI is added once there is something meaningful to check

---

## Phase 1: Core data model

Define the foundational dataframe abstractions and invariants.

Target outcomes:

- Dataframe construction
- Schema representation
- Column representation
- Data types
- Equal-length column validation
- Duplicate column name rejection
- Shape and schema inspection

---

## Phase 2: Inspection and projection

Add basic ways to inspect and subset dataframes.

Target outcomes:

- `head`
- `tail`
- row-like display/testing representation
- column selection
- column order preservation

---

## Phase 3: Expressions and filtering

Introduce expression objects and use them for row filtering.

Target outcomes:

- column references
- literals
- comparisons
- boolean expressions
- arithmetic expressions
- null checks
- filter operation

---

## Phase 4: Derived columns and sorting

Support expression-based column creation and row reordering.

Target outcomes:

- add/replace columns
- preserve immutability
- sort by column
- define null sort behaviour

---

## Phase 5: CSV I/O

Add basic CSV import/export.

Target outcomes:

- CSV writing
- CSV reading with explicit schema
- CSV schema inference
- strict parse errors
- null parsing rules

---

## Phase 6: Aggregation

Support dataframe-wide and grouped aggregation.

Target outcomes:

- `sum`
- `mean`
- `min`
- `max`
- `count(column)`
- `count(*)`
- group-by aggregation
- null aggregation semantics

---

## Phase 7: Joins

Support basic relational joins.

Target outcomes:

- inner join
- left join
- key type validation
- duplicate non-key column handling
- null join semantics

---

## Phase 8: Refactor checkpoint

Review the design before adding more ambitious extensions.

Target outcomes:

- clean public API
- structured errors
- stable expression model
- storage/operation separation reviewed
- documentation updated
- next phase chosen