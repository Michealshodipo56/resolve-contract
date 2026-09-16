#!/usr/bin/env bash
# Create planned contributor issues for Resolve repos (requires gh auth).
# Usage: ./scripts/create-github-issues.sh <repo>   e.g. resolve-protocol/resolve-contract
set -euo pipefail

REPO="${1:?usage: $0 owner/repo}"

create_issue() {
  local title="$1"
  local body="$2"
  local labels="$3"
  gh issue create --repo "$REPO" --title "$title" --label "$labels" --body "$body"
}

create_issue "docs: add worked payout examples to README" \
"## Summary
Expand protocol docs with additional worked examples (tiny amounts, hedged positions, dust).

## Acceptance Criteria
- [ ] At least three numeric examples covering payout, refund, and dust
- [ ] Examples match on-chain floor division rules
- [ ] Linked from main README

## Tech Stack
Markdown" \
"documentation,good first issue"

create_issue "feat: optional market metadata URI field" \
"## Summary
Allow creators to attach an HTTPS/IPFS metadata URI for richer off-chain context without enlarging on-chain question strings.

## Acceptance Criteria
- [ ] Spec for max length and allowed schemes
- [ ] Contract field + event update (non-breaking additive if possible)
- [ ] SDK + indexer + app surface the URI
- [ ] Tests for validation

## Tech Stack
Rust/Soroban, TypeScript SDK" \
"enhancement"

create_issue "feat: indexer backfill question via get_market RPC" \
"## Summary
After market_created, optionally invoke get_market to populate question/description in the indexer DB.

## Acceptance Criteria
- [ ] Config flag to enable/disable backfill
- [ ] Idempotent updates
- [ ] Failure does not block ingest checkpoint
- [ ] Tests with mocked RPC

## Tech Stack
TypeScript, SQLite" \
"enhancement"

create_issue "test: property tests for payout solvency" \
"## Summary
Add proptest-style tests that random valid pool splits never over-distribute.

## Acceptance Criteria
- [ ] Property: sum of floor payouts <= total pool
- [ ] Property: refunds equal deposits
- [ ] Runs in CI under reasonable time

## Tech Stack
Rust, soroban-sdk testutils / proptest" \
"testing"

create_issue "chore: mainnet deployment runbook" \
"## Summary
Document mainnet checklist: SAC token choice, resolver ops, monitoring, incident response.

## Acceptance Criteria
- [ ] Runbook markdown merged
- [ ] Links to explorer verification steps
- [ ] Explicit unaudited warning retained

## Tech Stack
Docs" \
"documentation"

echo "Issues created on $REPO"
