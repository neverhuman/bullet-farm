# Bullet Farm IEEE / arXiv preprint

Status: **preprint source; not release evidence**  
Last reviewed: 2026-08-25 (R3: Hub implementation `6eb002b`, code/report subject `0cc7eec`; sibling inventory `0346fd5` / `236f4ef` / `8272844`; Jankurai 58/raw58, 10 caps, 44 findings = 29 high/hard + 15 medium/soft; F1–F11 red-team corrections; 26/26 release gates blocked; compile is not a release)

This directory holds the citable IEEEtran source for:

> Bullet Farm: Architecture and Pre-Release Inventory of a Software-Change Transaction Processor

Authors are a placeholder (`Bullet Farm Maintainers`) until named authors are
supplied for an arXiv upload. A green compile does not authorize a command,
Candidate, Evidence result, effect, installer, or release.

## Build

From this directory, with TeX Live (IEEEtran, TikZ, booktabs, cite):

```bash
pdflatex bullet_farm_ieee.tex
bibtex bullet_farm_ieee
pdflatex bullet_farm_ieee.tex
pdflatex bullet_farm_ieee.tex
```

The PDF is a generated artifact and is not a family release subject. Do not
treat page count, reference count, or a clean compile as product completeness.

## Authority

Write inventory rows only from
[`../assurance/v1-closure-plan.md`](../assurance/v1-closure-plan.md),
[`../release.md`](../release.md), and the operator index
[`../assurance/product-gaps.md`](../assurance/product-gaps.md). Pin competitor subjects from
[`../assurance/competitor-snapshot.md`](../assurance/competitor-snapshot.md)
and Appendix A of the paper. Historical Centerrail material under
[`../spec/`](../spec/) is provenance, not runtime fact.

The earlier separately produced IEEE draft recorded in
[`../spec/paper.md`](../spec/paper.md) is not distributed here. Its hashes do
not verify these files.
