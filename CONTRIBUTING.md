# Contributing to resolve-contract

Thanks for helping improve Resolve.

## Development

1. Install Rust 1.84+ and `wasm32v1-none`.
2. Run `cargo test --manifest-path contracts/resolve/Cargo.toml`.
3. Prefer small, focused PRs (one logical change).

## Scope

This repo is Rust/Soroban only. SDK, indexer, and UI changes belong in their sibling repositories. If a change spans layers, open linked PRs and note dependencies.

## Pull requests

- Describe the protocol impact (state machine, auth, math, events).
- Add or update tests for any behavior change.
- Do not renumber `Error` enum discriminants (ABI).
- Do not add upgrade/admin backdoors without an explicit design discussion.

## Issues

Use clear titles. Include Summary, Acceptance Criteria (checkboxes), and whether the work touches auth, storage, or payout math.
