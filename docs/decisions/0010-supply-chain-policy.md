# ADR 0010: Supply-chain and release policy

Status: Accepted
Owner: Bullet Farm maintainers
Last reviewed: 2026-08-24
Applies to: build, CI, and release

## Decision

Pin Rust, Node, Java/TLC, actions, dependencies, and certified external binaries. Required builds
are locked/offline where supported. Secret, dependency, vulnerability, source, and license scanners
are installed and blocking. Trust-boundary parsers receive fuzz/property/sanitizer coverage.

Releases produce signed SBOM, SLSA provenance, reproducibility evidence, and immutable artifacts.
Signer/builder expectations, revocation, supported versions, disclosure, and rollback are explicit.

## Consequence

Missing tools and scanner failures are failures, not skips. Wave 2 supply-chain acceptance remains
required before a production claim.
