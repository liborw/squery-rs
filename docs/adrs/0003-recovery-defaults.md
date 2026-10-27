# ADR-0003: Recovery defaults

- Status: Proposed
- Date: 2026-10-27

## Context
A prototype needs clear behavior for ambiguous and unfinished tokens.

## Decision
Skip unfinished filters and tokens with unclosed quotes or dangling escapes, report warnings, and preserve valid neighboring tokens. Treat unrecognized field shapes as text.

## Rationale
Simple and reversible while preserving valid results.

## Consequences
No partial syntax nodes yet; some malformed field-like text remains free text.

## Alternatives considered
Partial AST nodes increase the public model before completion is implemented.
