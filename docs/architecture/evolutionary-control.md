# Evolutionary multi-agent control

Status: **normative V1 design; runtime coverage is incomplete**  
Owner: Bullet Farm maintainers  
Last reviewed: 2026-08-25

This document defines the safe meaning of "evolutionary" in Bullet Farm. It is
bounded search over immutable, evidence-bearing software-change lineages. It is
not autonomous credential use, self-reported fitness, unbounded agent debate,
or online self-modification.

## Objectives

The control plane should increase useful parallel exploration while preserving
five non-negotiable properties:

1. one durable authority decides who may mutate each exact subject;
2. every change has reconstructable inputs, lineage, Candidate, and receipts;
3. independent verification outranks writer or provider claims;
4. effects are idempotent, read back, and reconciled after ambiguity; and
5. cost, quota, risk, and verifier capacity bound every generation.

Hard constraints are filters, never terms that a high score may offset.

## Evolutionary vocabulary

| Term | V1 meaning |
| --- | --- |
| Mission | durable user objective, constraints, repository subjects, and acceptance policy |
| Graph Revision | immutable dependency graph and package decomposition admitted for the Mission |
| Selection Group | bounded set of Variants evaluated against the same declared objective and subject |
| Variant | immutable hypothesis, role/profile/routing/context/configuration snapshot, and parent lineage |
| Attempt | one execution incarnation under a unique lease and monotonically increasing fence |
| Proposal | provider-produced typed patch data; never shell, Git, Evidence, or effect authority |
| Candidate | provenance-bound immutable software-change phenotype created by BulletGit |
| Evidence | independently reproduced result over the exact Candidate and admitted gate definitions |
| Fitness Record | policy evaluation over Evidence, cost, risk, and declared observations |
| Fusion | a new Variant derived from named parents and a persisted dissent/fusion decision |
| Outcome | observed post-integration result; it does not rewrite historical fitness |

The genotype is the immutable Variant input record. The phenotype is its exact
Candidate plus Evidence. Rebase, policy, toolchain, environment, scope, gate, or
repository-subject changes create a different identity.

## Roles are capabilities, not personas

Roles select allowed operations, required inputs, output schemas, budgets, and
independence constraints. They never carry credentials.

| Role | Allowed output | Prohibited authority |
| --- | --- | --- |
| Planner | typed graph/package proposal and uncertainties | repository mutation or completion |
| Researcher | cited observations and context capsule additions | engineering truth or effects |
| Implementer | `PatchProposal` against an exact checkpoint | direct filesystem/Git/effect writes |
| Critic / red team | counterexamples, risk findings, dissent record | editing the Candidate it evaluates |
| Fusion | typed derivation from named parent Candidates | silently rewriting or erasing a parent |
| Verifier | clean-reconstruction Evidence | writer workspace or writer-produced proof |
| Effect broker | intent, dispatch, read-back, reconciliation receipt | provider execution or Candidate selection |
| Observer | post-integration measurements | retrospective mutation of Evidence |

A concrete agent may serve different roles in different Attempts, but an
Evidence gate requiring independence rejects the same writer trust context.
Role diversity without trust separation is not independent review.

## Lifecycle and state transitions

```text
Mission + Graph Revision + policy snapshot
            |
            v
   hard-constraint routing
            |
            v
 Selection Group (bounded Variants)
      | fenced Attempts |
      v                 v
 typed proposals -> exact Candidates
             \          /
              independent Evidence
                       |
              deterministic selection
                 /       |       \
              reject   select   fuse -> new lineage
                         |
                 authorized Effect
                         |
             read-back + reconciliation
                         |
               observation window
```

Kernel persists every transition and its request digest. Only BulletGit applies
a proposal or constructs a Candidate. Only a verifier result over a clean exact
reconstruction may satisfy an independent gate. Only the effect broker owns
external effect credentials. Portal state is downstream projection.

## Routing and exploration

Routing applies in this order:

1. reject incompatible repository, graph, package, scope, policy, data,
   containment, provider, profile, quota, or deadline constraints;
2. reserve known quota and budget transactionally;
3. enforce verifier and effect-broker backpressure;
4. rank the eligible set using the persisted deterministic routing policy; and
5. record considered choices, exclusions, tie-break inputs, and selected route.

`UNKNOWN` paid quota blocks dispatch. A narrowly scoped read-only probe is
allowed only by explicit policy and its own reservation. Diversity constraints
may request different provider/profile/role combinations, but never bypass a
hard filter.

Every Selection Group declares maximum Variants, concurrent Attempts, repair
loops, tokens, cost, wall time, context bytes, and verifier backlog. V1 permits
at most two provider repair loops. Exhaustion raises a typed struggle or
intervention; it does not silently expand authority.

## Fitness and selection

Fitness is a vector of independently sourced observations, not one opaque
number. A policy may include:

- gate outcome and Evidence tier;
- policy/security findings and changed-risk surface;
- regression and mutation-test strength where admitted;
- patch size, complexity delta, and reviewability;
- runtime, token, monetary, and verifier cost;
- novelty or dissent coverage relative to other Variants; and
- observation-window reliability after integration.

Required gates, forbidden effects, stale subjects, unknown authority, and
unsupported containment are disqualifying. Among eligible Candidates, the
frozen policy computes a Pareto frontier or lexicographic order and applies a
documented stable tie-break such as Candidate ID. Replaying the same records and
policy must choose the same result.

Provider confidence, eloquence, vote count, and writer-authored test output are
observations at most. They cannot satisfy independent correctness or safety
dimensions. Missing or contradictory observations remain `UNKNOWN` and trigger
the policy's declared escalation path.

## Fusion and dissent

Fusion is a new derivation with:

- ordered parent Candidate IDs and proof roots;
- the exact dissent statements and conflicts being resolved;
- a typed fusion proposal and fresh Attempt/fence;
- a new Candidate identity and independent verification; and
- no inherited PASS after any subject change.

Losing and superseded Variants remain immutable for audit and later analysis.
No process edits a winning Candidate in place, averages incompatible patches,
or treats consensus as correctness.

## Failure, backpressure, and recovery

Heartbeat failure, zero-row renewal, expiry, or supersession freezes mutation,
kills the provider process tree, and preserves the workspace before cleanup.
A successor receives a higher fence and resumes only from an exact checkpoint.
Stale writers cannot publish.

Verifier queues have explicit capacity. When saturated, Kernel stops admitting
new writer work before it compromises independence or creates unbounded stale
Candidates. Repeated failure, contradictory evidence, quota uncertainty,
selection instability, or an exhausted repair budget produces a typed
`Intervention` with the exact blocking record.

Remote success followed by a lost response becomes `UNKNOWN`. The effect broker
reads back the exact idempotency subject and either adopts the original effect
or reports a conflict; it never retries a write merely because transport timed
out.

## V1 adaptation boundary

V1 persists task, role/profile, routing, context, proposal, dissent, Evidence,
cost, and outcome provenance. Its routing and selection policies are versioned,
reviewed, deterministic inputs. It does not update weights, prompts, policies,
or privileges online.

Contextual routing learners, automatic role evolution, cross-repository sagas,
and larger councils are post-V1. They require holdouts, causal evaluation,
rollback, drift detection, guardrail non-regression, and an explainable signed
decision record before influencing production admission.

## Acceptance properties

The implementation is not conformant until tests prove:

- concurrent Variant acquisition yields one active writer and unique fences;
- any authority-field mutation, replay, expiry, supersession, or authority
  outage yields zero unauthorized mutation;
- Candidate identity changes for every bound genotype/phenotype field;
- selection replay is deterministic and hard constraints cannot be outweighed;
- fusion preserves parent/dissent lineage and invalidates inherited proof;
- writer evidence cannot satisfy independent gates;
- quota, budget, repair, and verifier-backlog limits stop new work; and
- ambiguous effects reconcile by read-back without a second write.

Current implementation status is tracked only in the
[V1 closure plan](../assurance/v1-closure-plan.md). This model is not a claim
that those runtime properties are already complete.
