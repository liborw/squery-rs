# squery

A dependency-free, platform-neutral Rust parser for GitHub-style search syntax. It extracts ordered free-text terms and field filters without interpreting their meaning. Incomplete input produces diagnostics while valid terms remain available.

## Quick start

```sh
cargo run --example basic
cargo test
```

```rust
let result = squery::parse("authentication status:open comments:>10 author:");
for filter in result.query.filters() {
    println!("{} {:?} {}", filter.field, filter.operator, filter.value);
}
assert_eq!(result.diagnostics.len(), 1); // author: incomplete
// For all-or-nothing consumers: result.into_strict()
```

## Prototype scope

Supported: free text, double-quoted terms and values, backslash escaping, `field:value`, `field:=value`, `field:!=value`, `field:>value`, `field:>=value`, `field:<value`, `field:<=value`, `-field:value`; UTF-8 byte ranges, ordered terms and recoverable diagnostics.

Not supported: field validation, types/dates, completion, execution, boolean groups, custom grammar, server integration.

## Architecture

`parse(&str) -> ParseResult` returns `Query { terms: Vec<Term> }` plus `Vec<Diagnostic>`. `Term` is `Text` or `Filter`. `ParseResult::into_strict()` rejects queries with diagnostics. See `docs/specification.md` and `docs/adrs/`.

## Verification

- `cargo test` — Not verified: Rust toolchain unavailable in this environment.
- `cargo run --example basic` — Not verified: Rust toolchain unavailable in this environment.
- `python scripts/validate_handoff.py /mnt/data/squery` — handoff validation performed.

## Known limitations

Incomplete quoted/escaped tokens are omitted rather than represented as partial nodes. Unknown or malformed field shapes are treated as text, since the crate has no domain registry. Diagnostics distinguish incomplete syntax, not type errors. The default error severity for incomplete input is Warning; strict consumption rejects any diagnostic.
