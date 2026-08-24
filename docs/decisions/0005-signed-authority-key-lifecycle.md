# ADR 0005: Signed authority and key lifecycle

Status: Accepted
Owner: Bullet Kernel maintainers
Last reviewed: 2026-08-24
Applies to: every mutation gateway

## Decision

Use Ed25519 signatures over a domain tag plus RFC 8785 claims. Envelopes bind issuer, audience,
operation, authenticated subject, full graph/repo/Attempt/fence/workspace/scope/config/policy/route
closure, request digest, authority and freeze generations, validity window, and unique nonce.

Keys have activation, expiry, rotation, revocation, and retention beyond maximum token/audit
lifetime. Gateways validate current policy and registered principal state, not only signatures.

## Consequence

Unsigned claims and placeholder digests never authorize. Tokens cannot travel through argv,
environment, URLs, browser storage, or ordinary logs. Implementation is a Wave 2 prerequisite.
