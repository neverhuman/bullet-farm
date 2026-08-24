------------------------------ MODULE EffectCheck ------------------------------
EXTENDS Naturals, FiniteSets, TLC

CONSTANTS MaxDispatches, MaxEpoch
ASSUME /\ MaxDispatches \in Nat \ {0}
       /\ MaxEpoch \in Nat \ {0, 1}

EffectPhases == {"absent", "intent", "dispatching", "unknown", "verified", "orphaned"}
CheckPhases == {"absent", "intent", "dispatching", "unknown", "verified", "orphaned"}
RemoteValues == {"none", "desired", "third_party"}

VARIABLES effectPhase, remoteEffect, effectDispatches, effectReadback,
          proofDurable, checkPhase, remoteCheck, checkDispatches,
          checkReadback, policyLive, frozen, authorityEpoch,
          logicalEffects, logicalChecks

vars == <<effectPhase, remoteEffect, effectDispatches, effectReadback,
          proofDurable, checkPhase, remoteCheck, checkDispatches,
          checkReadback, policyLive, frozen, authorityEpoch,
          logicalEffects, logicalChecks>>

(*
--algorithm EffectCheckProtocol {
variables effectPhase = "absent", remoteEffect = "none", effectDispatches = 0,
          effectReadback = FALSE, proofDurable = FALSE, checkPhase = "absent",
          remoteCheck = "none", checkDispatches = 0, checkReadback = FALSE,
          policyLive = TRUE, frozen = FALSE, authorityEpoch = 1,
          logicalEffects = 0, logicalChecks = 0;
begin Loop:
  while (TRUE) {
    either when effectPhase = "absent" /\ policyLive /\ ~frozen;
      effectPhase := "intent";
    or when effectPhase = "intent" /\ remoteEffect = "none" /\ policyLive /\ ~frozen;
      effectPhase := "dispatching" || effectDispatches := effectDispatches + 1;
    or when effectPhase = "dispatching";
      remoteEffect := "desired" || logicalEffects := 1 || effectPhase := "unknown";
    or when effectPhase = "unknown";
      effectReadback := TRUE;
    or skip;
    end either;
  }
end algorithm;
*)

Init ==
    /\ effectPhase = "absent"
    /\ remoteEffect = "none"
    /\ effectDispatches = 0
    /\ effectReadback = FALSE
    /\ proofDurable = FALSE
    /\ checkPhase = "absent"
    /\ remoteCheck = "none"
    /\ checkDispatches = 0
    /\ checkReadback = FALSE
    /\ policyLive = TRUE
    /\ frozen = FALSE
    /\ authorityEpoch = 1
    /\ logicalEffects = 0
    /\ logicalChecks = 0

PersistEffectIntent ==
    /\ effectPhase = "absent"
    /\ policyLive
    /\ ~frozen
    /\ effectPhase' = "intent"
    /\ UNCHANGED <<remoteEffect, effectDispatches, effectReadback,
                    proofDurable, checkPhase, remoteCheck, checkDispatches,
                    checkReadback, policyLive, frozen, authorityEpoch,
                    logicalEffects, logicalChecks>>

DispatchEffect ==
    /\ effectPhase = "intent"
    /\ remoteEffect = "none"
    /\ policyLive
    /\ ~frozen
    /\ effectDispatches < MaxDispatches
    /\ effectPhase' = "dispatching"
    /\ effectDispatches' = effectDispatches + 1
    /\ effectReadback' = FALSE
    /\ UNCHANGED <<remoteEffect, proofDurable, checkPhase, remoteCheck,
                    checkDispatches, checkReadback, policyLive, frozen,
                    authorityEpoch, logicalEffects, logicalChecks>>

EffectResponseSuccess ==
    /\ effectPhase = "dispatching"
    /\ remoteEffect' = "desired"
    /\ logicalEffects' = 1
    /\ effectPhase' = "verified"
    /\ effectReadback' = TRUE
    /\ UNCHANGED <<effectDispatches, proofDurable, checkPhase, remoteCheck,
                    checkDispatches, checkReadback, policyLive, frozen,
                    authorityEpoch, logicalChecks>>

EffectResponseLost ==
    /\ effectPhase = "dispatching"
    /\ remoteEffect' = "desired"
    /\ logicalEffects' = 1
    /\ effectPhase' = "unknown"
    /\ UNCHANGED <<effectDispatches, effectReadback, proofDurable, checkPhase,
                    remoteCheck, checkDispatches, checkReadback, policyLive,
                    frozen, authorityEpoch, logicalChecks>>

EffectTimeoutWithoutMutation ==
    /\ effectPhase = "dispatching"
    /\ remoteEffect = "none"
    /\ effectPhase' = "unknown"
    /\ UNCHANGED <<remoteEffect, effectDispatches, effectReadback,
                    proofDurable, checkPhase, remoteCheck, checkDispatches,
                    checkReadback, policyLive, frozen, authorityEpoch,
                    logicalEffects, logicalChecks>>

ThirdPartyEffect ==
    /\ effectPhase \in {"intent", "dispatching", "unknown"}
    /\ remoteEffect = "none"
    /\ remoteEffect' = "third_party"
    /\ UNCHANGED <<effectPhase, effectDispatches, effectReadback,
                    proofDurable, checkPhase, remoteCheck, checkDispatches,
                    checkReadback, policyLive, frozen, authorityEpoch,
                    logicalEffects, logicalChecks>>

ReadBackEffect ==
    /\ effectPhase = "unknown"
    /\ effectReadback' = TRUE
    /\ IF remoteEffect = "desired"
          THEN /\ effectPhase' = "verified" /\ logicalEffects' = 1
          ELSE IF remoteEffect = "third_party"
          THEN /\ effectPhase' = "orphaned" /\ logicalEffects' = logicalEffects
          ELSE /\ effectPhase' = "intent" /\ logicalEffects' = logicalEffects
    /\ UNCHANGED <<remoteEffect, effectDispatches, proofDurable, checkPhase,
                    remoteCheck, checkDispatches, checkReadback, policyLive,
                    frozen, authorityEpoch, logicalChecks>>

PersistProof ==
    /\ effectPhase = "verified"
    /\ effectReadback
    /\ proofDurable' = TRUE
    /\ UNCHANGED <<effectPhase, remoteEffect, effectDispatches, effectReadback,
                    checkPhase, remoteCheck, checkDispatches, checkReadback,
                    policyLive, frozen, authorityEpoch, logicalEffects,
                    logicalChecks>>

PersistCheckIntent ==
    /\ proofDurable
    /\ effectPhase = "verified"
    /\ checkPhase = "absent"
    /\ policyLive
    /\ ~frozen
    /\ checkPhase' = "intent"
    /\ UNCHANGED <<effectPhase, remoteEffect, effectDispatches, effectReadback,
                    proofDurable, remoteCheck, checkDispatches, checkReadback,
                    policyLive, frozen, authorityEpoch, logicalEffects,
                    logicalChecks>>

DispatchCheck ==
    /\ checkPhase = "intent"
    /\ remoteCheck = "none"
    /\ policyLive
    /\ ~frozen
    /\ checkDispatches < MaxDispatches
    /\ checkPhase' = "dispatching"
    /\ checkDispatches' = checkDispatches + 1
    /\ checkReadback' = FALSE
    /\ UNCHANGED <<effectPhase, remoteEffect, effectDispatches, effectReadback,
                    proofDurable, remoteCheck, policyLive, frozen,
                    authorityEpoch, logicalEffects, logicalChecks>>

CheckResponseLost ==
    /\ checkPhase = "dispatching"
    /\ remoteCheck' = "desired"
    /\ logicalChecks' = 1
    /\ checkPhase' = "unknown"
    /\ UNCHANGED <<effectPhase, remoteEffect, effectDispatches, effectReadback,
                    proofDurable, checkDispatches, checkReadback, policyLive,
                    frozen, authorityEpoch, logicalEffects>>

CheckTimeoutWithoutMutation ==
    /\ checkPhase = "dispatching"
    /\ remoteCheck = "none"
    /\ checkPhase' = "unknown"
    /\ UNCHANGED <<effectPhase, remoteEffect, effectDispatches, effectReadback,
                    proofDurable, remoteCheck, checkDispatches, checkReadback,
                    policyLive, frozen, authorityEpoch, logicalEffects,
                    logicalChecks>>

ThirdPartyCheck ==
    /\ checkPhase \in {"intent", "dispatching", "unknown"}
    /\ remoteCheck = "none"
    /\ remoteCheck' = "third_party"
    /\ UNCHANGED <<effectPhase, remoteEffect, effectDispatches, effectReadback,
                    proofDurable, checkPhase, checkDispatches, checkReadback,
                    policyLive, frozen, authorityEpoch, logicalEffects,
                    logicalChecks>>

ReadBackCheck ==
    /\ checkPhase = "unknown"
    /\ checkReadback' = TRUE
    /\ IF remoteCheck = "desired"
          THEN /\ checkPhase' = "verified" /\ logicalChecks' = 1
          ELSE IF remoteCheck = "third_party"
          THEN /\ checkPhase' = "orphaned" /\ logicalChecks' = logicalChecks
          ELSE /\ checkPhase' = "intent" /\ logicalChecks' = logicalChecks
    /\ UNCHANGED <<effectPhase, remoteEffect, effectDispatches, effectReadback,
                    proofDurable, remoteCheck, checkDispatches, policyLive,
                    frozen, authorityEpoch, logicalEffects>>

ExpirePolicy ==
    /\ policyLive
    /\ policyLive' = FALSE
    /\ UNCHANGED <<effectPhase, remoteEffect, effectDispatches, effectReadback,
                    proofDurable, checkPhase, remoteCheck, checkDispatches,
                    checkReadback, frozen, authorityEpoch, logicalEffects,
                    logicalChecks>>

Freeze ==
    /\ ~frozen
    /\ frozen' = TRUE
    /\ UNCHANGED <<effectPhase, remoteEffect, effectDispatches, effectReadback,
                    proofDurable, checkPhase, remoteCheck, checkDispatches,
                    checkReadback, policyLive, authorityEpoch, logicalEffects,
                    logicalChecks>>

Restore ==
    /\ authorityEpoch < MaxEpoch
    /\ authorityEpoch' = authorityEpoch + 1
    /\ frozen' = TRUE
    /\ UNCHANGED <<effectPhase, remoteEffect, effectDispatches, effectReadback,
                    proofDurable, checkPhase, remoteCheck, checkDispatches,
                    checkReadback, policyLive, logicalEffects, logicalChecks>>

Crash == UNCHANGED vars

Next ==
    \/ PersistEffectIntent \/ DispatchEffect \/ EffectResponseSuccess
    \/ EffectResponseLost \/ EffectTimeoutWithoutMutation \/ ThirdPartyEffect
    \/ ReadBackEffect \/ PersistProof \/ PersistCheckIntent \/ DispatchCheck
    \/ CheckResponseLost \/ CheckTimeoutWithoutMutation \/ ThirdPartyCheck
    \/ ReadBackCheck \/ ExpirePolicy \/ Freeze \/ Restore \/ Crash

TypeOK ==
    /\ effectPhase \in EffectPhases
    /\ remoteEffect \in RemoteValues
    /\ effectDispatches \in 0..MaxDispatches
    /\ effectReadback \in BOOLEAN
    /\ proofDurable \in BOOLEAN
    /\ checkPhase \in CheckPhases
    /\ remoteCheck \in RemoteValues
    /\ checkDispatches \in 0..MaxDispatches
    /\ checkReadback \in BOOLEAN
    /\ policyLive \in BOOLEAN
    /\ frozen \in BOOLEAN
    /\ authorityEpoch \in 1..MaxEpoch
    /\ logicalEffects \in 0..1
    /\ logicalChecks \in 0..1

AtMostOneLogicalEffect == logicalEffects <= 1 /\ logicalChecks <= 1
VerifiedEffectWasReadBack == effectPhase = "verified" => effectReadback
CheckRequiresDurableProof == checkPhase # "absent" => proofDurable /\ effectPhase = "verified"
ThirdPartyStateIsNeverAdopted ==
    /\ (remoteEffect = "third_party" => effectPhase # "verified")
    /\ (remoteCheck = "third_party" => checkPhase # "verified")
NoDispatchAfterStop == (~policyLive \/ frozen) =>
    /\ effectPhase # "dispatching"
    /\ checkPhase # "dispatching"

Spec == Init /\ [][Next]_vars

=============================================================================
