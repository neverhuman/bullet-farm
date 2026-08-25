# Bullet Farm paper and executive brief

Status: **Stage-1 architecture/component-assurance preprint; not release evidence**
Last reviewed: 2026-08-25

This directory holds the citable IEEEtran paper and one-column executive brief:

> Bullet Farm: A Software-Change Transaction Processor for Heterogeneous Agents

The author is **Bullet Farm Maintainers**. A green compile does not authorize a
command, Candidate, Evidence result, effect, installer, or release.

## Build and preflight

From the Hub root:

```bash
just paper
just paper-check
```

`just paper` generates shared LaTeX macros from `evidence.json` and builds both
PDFs under one source epoch. `just paper-check` is strict: it rejects dirty or
mismatched subjects, stale input hashes, bibliography/reference problems,
layout warnings, missing metadata, unembedded fonts, wrong page size/count, and
nondeterministic bytes. `PAPER_ALLOW_DIRTY=1 bash scripts/paper-check.sh` exists
only for local layout development; it is not publication preflight.

The evidence manifest follows `bullet.paper-evidence.v1`. Both documents
consume `evidence.generated.tex`; edit `evidence.json`, never generated macros.
PDF hashes are reported by the successful strict check. Neither PDF is a family
release subject.

## Authority

Write inventory rows only from
[`../assurance/v1-closure-plan.md`](../assurance/v1-closure-plan.md),
[`../release.md`](../release.md), and the operator index
[`../assurance/product-gaps.md`](../assurance/product-gaps.md). Pin competitor
subjects from
[`../assurance/competitor-snapshot.md`](../assurance/competitor-snapshot.md)
and the dated landscape appendix. The non-authoritative opportunity backlog is
[`../workplan.md`](../workplan.md); it cannot change the closure plan.

Stage 2, including any matched benchmark or measured superiority claim, is
deferred until a signed connected `TRANSACTION_PROOF` exists. Historical
Centerrail material under [`../spec/`](../spec/) remains provenance, not
runtime fact.
