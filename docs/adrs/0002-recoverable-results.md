# ADR-0002: Recoverable parse result

- Status: Accepted
- Date: 2026-10-27

## Context
Interactive typing produces unfinished filters; errors should not discard preceding valid syntax.

## Decision
`parse` always returns both an ordered query and diagnostics; `into_strict` is optional.

## Rationale
One parsing implementation serves interactive and strict consumers.

## Consequences
Consumers must inspect diagnostics before executing a complete query.

## Alternatives considered
`Result<Query, Error>` loses valid terms on the first syntax error.
