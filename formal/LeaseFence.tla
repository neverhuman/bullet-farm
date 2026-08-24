------------------------------ MODULE LeaseFence ------------------------------
EXTENDS Naturals, FiniteSets, TLC

CONSTANTS Runners, MaxFence, MaxTime, MaxScope, MaxOps, MaxEpoch, MaxFreeze

ASSUME /\ Runners # {}
       /\ MaxFence \in Nat \ {0}
       /\ MaxTime \in Nat \ {0}
       /\ MaxScope \in Nat \ {0}
       /\ MaxOps \in Nat \ {0}
       /\ MaxEpoch \in Nat \ {0, 1}
       /\ MaxFreeze \in Nat \ {0}

None == "none"

VARIABLES now, leaseOwner, fence, expires, tokenFence, tokenEpoch,
          authorityEpoch, freezeGeneration, frozen, scopeRevision,
          ackRevision, barrier, inFlight, acceptedApplies, refusedApplies

vars == <<now, leaseOwner, fence, expires, tokenFence, tokenEpoch,
          authorityEpoch, freezeGeneration, frozen, scopeRevision,
          ackRevision, barrier, inFlight, acceptedApplies, refusedApplies>>

(*
--algorithm LeaseFenceProtocol {
variables now = 0, leaseOwner = "none", fence = 0, expires = 0,
          tokenFence = [r \in Runners |-> 0],
          tokenEpoch = [r \in Runners |-> 0], authorityEpoch = 1,
          freezeGeneration = 0, frozen = FALSE, scopeRevision = 1,
          ackRevision = 0, barrier = FALSE, inFlight = 0,
          acceptedApplies = 0, refusedApplies = 0;
process (runner \in Runners) {
Loop:
  while (TRUE) {
    either when leaseOwner = "none" /\ ~frozen /\ fence < MaxFence;
      with (nextFence = fence + 1) {
        leaseOwner := self || fence := nextFence || expires := now + 1 ||
        tokenFence[self] := nextFence || tokenEpoch[self] := authorityEpoch ||
        ackRevision := scopeRevision;
      };
    or when now < MaxTime; now := now + 1;
    or when leaseOwner = self /\ expires > now /\ ~frozen;
      expires := IF now < MaxTime THEN now + 1 ELSE now;
    or when leaseOwner # "none" /\ expires <= now /\ inFlight = 0;
      leaseOwner := "none" || barrier := FALSE || ackRevision := 0;
    or skip;
    end either;
  }
}
}
*)

Init ==
    /\ now = 0
    /\ leaseOwner = None
    /\ fence = 0
    /\ expires = 0
    /\ tokenFence = [r \in Runners |-> 0]
    /\ tokenEpoch = [r \in Runners |-> 0]
    /\ authorityEpoch = 1
    /\ freezeGeneration = 0
    /\ frozen = FALSE
    /\ scopeRevision = 1
    /\ ackRevision = 0
    /\ barrier = FALSE
    /\ inFlight = 0
    /\ acceptedApplies = 0
    /\ refusedApplies = 0

Authorized(r) ==
    /\ leaseOwner = r
    /\ tokenFence[r] = fence
    /\ tokenEpoch[r] = authorityEpoch
    /\ expires > now
    /\ ~frozen
    /\ ~barrier
    /\ ackRevision = scopeRevision

Acquire(r) ==
    /\ leaseOwner = None
    /\ ~frozen
    /\ fence < MaxFence
    /\ leaseOwner' = r
    /\ fence' = fence + 1
    /\ expires' = IF now < MaxTime THEN now + 1 ELSE now
    /\ tokenFence' = [tokenFence EXCEPT ![r] = fence + 1]
    /\ tokenEpoch' = [tokenEpoch EXCEPT ![r] = authorityEpoch]
    /\ ackRevision' = scopeRevision
    /\ UNCHANGED <<now, authorityEpoch, freezeGeneration, frozen,
                    scopeRevision, barrier, inFlight, acceptedApplies,
                    refusedApplies>>

Heartbeat(r) ==
    /\ leaseOwner = r
    /\ tokenFence[r] = fence
    /\ tokenEpoch[r] = authorityEpoch
    /\ expires > now
    /\ ~frozen
    /\ expires' = IF now < MaxTime THEN now + 1 ELSE now
    /\ UNCHANGED <<now, leaseOwner, fence, tokenFence, tokenEpoch,
                    authorityEpoch, freezeGeneration, frozen, scopeRevision,
                    ackRevision, barrier, inFlight, acceptedApplies,
                    refusedApplies>>

Tick ==
    /\ now < MaxTime
    /\ now' = now + 1
    /\ UNCHANGED <<leaseOwner, fence, expires, tokenFence, tokenEpoch,
                    authorityEpoch, freezeGeneration, frozen, scopeRevision,
                    ackRevision, barrier, inFlight, acceptedApplies,
                    refusedApplies>>

Expire ==
    /\ leaseOwner # None
    /\ expires <= now
    /\ inFlight = 0
    /\ leaseOwner' = None
    /\ ackRevision' = 0
    /\ barrier' = FALSE
    /\ UNCHANGED <<now, fence, expires, tokenFence, tokenEpoch,
                    authorityEpoch, freezeGeneration, frozen, scopeRevision,
                    inFlight, acceptedApplies, refusedApplies>>

EnterBarrier(r) ==
    /\ Authorized(r)
    /\ inFlight = 0
    /\ barrier' = TRUE
    /\ UNCHANGED <<now, leaseOwner, fence, expires, tokenFence, tokenEpoch,
                    authorityEpoch, freezeGeneration, frozen, scopeRevision,
                    ackRevision, inFlight, acceptedApplies, refusedApplies>>

AppendScope(r) ==
    /\ leaseOwner = r
    /\ barrier
    /\ inFlight = 0
    /\ scopeRevision < MaxScope
    /\ scopeRevision' = scopeRevision + 1
    /\ UNCHANGED <<now, leaseOwner, fence, expires, tokenFence, tokenEpoch,
                    authorityEpoch, freezeGeneration, frozen, ackRevision,
                    barrier, inFlight, acceptedApplies, refusedApplies>>

AcknowledgeScope(r) ==
    /\ leaseOwner = r
    /\ barrier
    /\ ackRevision' = scopeRevision
    /\ UNCHANGED <<now, leaseOwner, fence, expires, tokenFence, tokenEpoch,
                    authorityEpoch, freezeGeneration, frozen, scopeRevision,
                    barrier, inFlight, acceptedApplies, refusedApplies>>

Resume(r) ==
    /\ leaseOwner = r
    /\ barrier
    /\ ackRevision = scopeRevision
    /\ barrier' = FALSE
    /\ UNCHANGED <<now, leaseOwner, fence, expires, tokenFence, tokenEpoch,
                    authorityEpoch, freezeGeneration, frozen, scopeRevision,
                    ackRevision, inFlight, acceptedApplies, refusedApplies>>

TryApply(r) ==
    /\ inFlight = 0
    /\ acceptedApplies + refusedApplies < MaxOps
    /\ IF Authorized(r)
          THEN /\ inFlight' = 1
               /\ acceptedApplies' = acceptedApplies + 1
               /\ refusedApplies' = refusedApplies
          ELSE /\ inFlight' = 0
               /\ acceptedApplies' = acceptedApplies
               /\ refusedApplies' = refusedApplies + 1
    /\ UNCHANGED <<now, leaseOwner, fence, expires, tokenFence, tokenEpoch,
                    authorityEpoch, freezeGeneration, frozen, scopeRevision,
                    ackRevision, barrier>>

CommitApply(r) ==
    /\ inFlight = 1
    /\ Authorized(r)
    /\ inFlight' = 0
    /\ UNCHANGED <<now, leaseOwner, fence, expires, tokenFence, tokenEpoch,
                    authorityEpoch, freezeGeneration, frozen, scopeRevision,
                    ackRevision, barrier, acceptedApplies, refusedApplies>>

AbortInvalidApply ==
    /\ inFlight = 1
    /\ (expires <= now \/ frozen \/ barrier \/ leaseOwner = None)
    /\ inFlight' = 0
    /\ UNCHANGED <<now, leaseOwner, fence, expires, tokenFence, tokenEpoch,
                    authorityEpoch, freezeGeneration, frozen, scopeRevision,
                    ackRevision, barrier, acceptedApplies, refusedApplies>>

Freeze ==
    /\ ~frozen
    /\ freezeGeneration < MaxFreeze
    /\ frozen' = TRUE
    /\ freezeGeneration' = freezeGeneration + 1
    /\ UNCHANGED <<now, leaseOwner, fence, expires, tokenFence, tokenEpoch,
                    authorityEpoch, scopeRevision, ackRevision, barrier,
                    inFlight, acceptedApplies, refusedApplies>>

Restore ==
    /\ inFlight = 0
    /\ authorityEpoch < MaxEpoch
    /\ freezeGeneration < MaxFreeze
    /\ authorityEpoch' = authorityEpoch + 1
    /\ leaseOwner' = None
    /\ frozen' = TRUE
    /\ freezeGeneration' = freezeGeneration + 1
    /\ ackRevision' = 0
    /\ barrier' = FALSE
    /\ UNCHANGED <<now, fence, expires, tokenFence, tokenEpoch,
                    scopeRevision, inFlight, acceptedApplies, refusedApplies>>

RecoverActive ==
    /\ frozen
    /\ leaseOwner = None
    /\ frozen' = FALSE
    /\ UNCHANGED <<now, leaseOwner, fence, expires, tokenFence, tokenEpoch,
                    authorityEpoch, freezeGeneration, scopeRevision,
                    ackRevision, barrier, inFlight, acceptedApplies,
                    refusedApplies>>

Next ==
    \/ Tick
    \/ Expire
    \/ Freeze
    \/ Restore
    \/ RecoverActive
    \/ AbortInvalidApply
    \/ \E r \in Runners:
        Acquire(r) \/ Heartbeat(r) \/ EnterBarrier(r) \/ AppendScope(r)
        \/ AcknowledgeScope(r) \/ Resume(r) \/ TryApply(r) \/ CommitApply(r)

TypeOK ==
    /\ now \in 0..MaxTime
    /\ leaseOwner \in Runners \cup {None}
    /\ fence \in 0..MaxFence
    /\ expires \in 0..(MaxTime + 1)
    /\ tokenFence \in [Runners -> 0..MaxFence]
    /\ tokenEpoch \in [Runners -> Nat]
    /\ authorityEpoch \in 1..MaxEpoch
    /\ freezeGeneration \in 0..MaxFreeze
    /\ frozen \in BOOLEAN
    /\ scopeRevision \in 1..MaxScope
    /\ ackRevision \in 0..MaxScope
    /\ barrier \in BOOLEAN
    /\ inFlight \in 0..1
    /\ acceptedApplies \in 0..MaxOps
    /\ refusedApplies \in 0..MaxOps

BarrierHasNoInFlightApply == barrier => inFlight = 0
RestoredAuthorityNeedsFreshAcquire == leaseOwner = None => inFlight = 0

Spec == Init /\ [][Next]_vars

=============================================================================
