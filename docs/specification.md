# Prototype specification

## Context
A reusable Rust crate called `squery` for turning GitHub-style search strings into syntax-only structured values.

## Problem
A text query can mix free text and structured qualifiers, and users may submit incomplete input while typing. Valid parts must remain usable.

## Goals
- Parse mixed text and filters independent of a domain.
- Preserve input order and source locations.
- Return diagnostics without discarding valid results.

## Non-goals
- Domain schemas or field validation
- Typed value/date extraction
- Query evaluation, SQL, platform adapters, or completion

## Requirements
### Functional
- **FR-001** (`Conversation requirement`): Parse free text, quoted phrases, qualifiers, comparisons and negation. Acceptance: basic example yields ordered `Term` values.
- **FR-002** (`Conversation requirement`): Parse without interpreting field names or values. Acceptance: `comments:>10` contains the string `"10"` and `Operator::Gt`.
- **FR-003** (`Conversation requirement`): Recover from incomplete inputs. Acceptance: `status:open author:` retains the first filter and reports a diagnostic for the second.
- **FR-004** (`Conversation requirement`): Return query terms in original order and preserve spans for diagnostics and future editors.
- **FR-005** (`Conversation requirement`): Provide strict consumption convenience without maintaining two parsers.
### Non-functional
- **NFR-001** (`Conversation requirement`): Rust and platform neutral, no web/database requirement.

## Interfaces and data
`parse(&str) -> ParseResult`, `Query { terms: Vec<Term> }`, `Term::{Text, Filter}`, `Filter { field, operator, value, negated, span }`, diagnostics with source ranges. Ranges are UTF-8 byte offsets.

## Prototype assumptions
- **A-001** (`Prototype assumption`): Only ASCII letters may start a field; subsequent characters can be ASCII letters/digits, `_`, `-`. Can be changed later.
- **A-002** (`Prototype assumption`): Incomplete expressions are omitted and diagnosed with Warning severity, regardless of input position. Strict conversion rejects any diagnostic.
- **A-003** (`Prototype assumption`): Unrecognized field shapes are free text; ambiguous syntax is not a domain error.
- **A-004** (`Prototype assumption`): One backslash escapes the following character and double quotes group whitespace; no preservation of quote marks in decoded values.

## Open questions
- **Q-001** (`Open question`): Whether to add partial syntax nodes for editor completion.
- **Q-002** (`Open question`): Whether to make `=`/`!=` distinct from `:` long term.
- **Q-003** (`Open question`): Should punctuation-only malformed qualifiers return diagnostics rather than text?

## Out of scope for this prototype
- Field and value metadata, completion, serialization, and boolean grouping.

## Acceptance scenario
Running `cargo run --example basic` prints parsed terms for `authentication status:open comments:>10 -is:draft author:` and a diagnostic for the unfinished `author:`.
