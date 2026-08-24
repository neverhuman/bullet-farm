# ADR 0003: Five trust planes and principal separation

Status: Accepted
Owner: Bullet Farm maintainers
Last reviewed: 2026-08-24
Applies to: all runtime work

## Decision

Use five authority planes: control, execution/repository writer, independent verification,
effect/attestation/integration, and observation/audit. Broker, attestor, integration worker, and
observer remain separate workload principals within the effect-facing responsibility. The portal
and evolution engine are projections/decision aids with no mutation credential.

Role names in a recipe never establish identity. One-host deployment uses distinct OS identities
and peer credentials; distributed deployment later uses distinct SPIFFE-compatible identities.

## Consequence

A plane may consume only grants addressed to its authenticated principal. Compromise of evolution,
portal, author, or broker cannot manufacture independent evidence or integration authority.
