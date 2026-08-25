# Bullet Farm IEEE / arXiv preprint

Status: **preprint source; not release evidence**  
Last reviewed: 2026-08-25

This directory holds the citable IEEEtran source for:

> Bullet Farm: A Transaction Processor for Verified Multi-Agent Software Engineering

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
[`../assurance/v1-closure-plan.md`](../assurance/v1-closure-plan.md) and
[`../release.md`](../release.md). Pin competitor subjects from
[`../assurance/competitor-snapshot.md`](../assurance/competitor-snapshot.md)
and Appendix A of the paper. Historical Centerrail material under
[`../spec/`](../spec/) is provenance, not runtime fact.

The earlier separately produced IEEE draft recorded in
[`../spec/paper.md`](../spec/paper.md) is not distributed here. Its hashes do
not verify these files.
