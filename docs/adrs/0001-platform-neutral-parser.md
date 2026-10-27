# ADR-0001: Platform-neutral syntax-only parser

- Status: Accepted
- Date: 2026-10-27

## Context
Consumers need structured GitHub-like search syntax without domain interpretation.

## Decision
Expose `parse` and domain-neutral AST values from a small Rust crate named `squery`. No schema, execution, or platform dependencies.

## Rationale
Can run on servers, native apps, or other Rust environments without framework coupling.

## Consequences
Consumers own validation and execution; values remain strings.

## Alternatives considered
Schema-aware parsing was intentionally excluded.
