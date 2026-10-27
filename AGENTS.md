# AGENTS.md

## Objective
Maintain a small, dependency-free, domain-neutral, error-recovering Rust search query parser.

## Source of truth
1. `docs/specification.md`
2. Accepted ADRs
3. README for usage
4. Tests and source

Proposed ADRs and prototype assumptions are not user-approved requirements.

## Repository map
- `src/lib.rs`: public types, tokenizer/parser and unit tests
- `examples/basic.rs`: smoke example
- `docs/specification.md`: requirements and assumptions
- `docs/adrs`: architectural decisions

## Commands
- `cargo test`
- `cargo run --example basic`
- `cargo fmt -- --check`

## Architecture boundaries
No schemas, validation, date conversion, query execution, transport, async runtimes, or completion implementation. Preserve input order and byte spans. Recover valid terms even when neighboring input is malformed.

## Implementation conventions
Keep public types small and parsing deterministic. Prefer stdlib over dependencies. Avoid breaking the public API without review.

## Verification expectations
Run tests, formatting, and demo before shipping; where Rust toolchain is unavailable, report as not verified. Run `python scripts/validate_handoff.py /mnt/data/squery` for repository contract.

## Known limitations and open questions
Operator variants beyond `:` and whether to preserve exact quoting/trivia are open. Partial nodes not currently produced.

## Next work
Verify in Rust-enabled environment; address recovered malformed input semantics after tests and consumer feedback.
