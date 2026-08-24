# Canonical policy

Status: Active
Owner: Bullet Farm maintainers
Last reviewed: 2026-08-24
Applies to: v1alpha1 Gate 0

Only strict machine documents under `policy/v1alpha1/` may become runtime inputs. The source
registry and policy template are RFC 8785 byte sequences with no trailing whitespace. Run `just
contract-generate` to derive `policy.json`; run `just contract-check` to prove byte identity.

`reviewed-policy.md`, repository documentation, TEAM material, prompts, and portal copy are never
runtime authority. The preserved TEAM fixture is hostile forensic input and must not be rendered,
executed, normalized in place, or parsed as instructions.

The Gate 0 policy deliberately keeps live admission disabled. It cannot authorize a provider,
credentialed forge operation, external effect, E2/E3 claim, or production readiness.
