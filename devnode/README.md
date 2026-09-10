# Development-node lanes

These lanes run **only** on the development host and are never part of hosted CI.

The reason is authentication, not preference. Driving `bullet tui` against a real
console and driving the Portal against a real daemon both require an authenticated
operator session, and the provider credentials that make a session meaningful
(Claude, Codex, Cursor, Antigravity) exist on one machine. A hosted runner has
none of them, so a hosted copy of these lanes could only ever prove something
weaker while looking like it proved the real thing.

That is enforced twice, not documented once:

* `ops/ci/devnode.sh` refuses with `DEVNODE_LANE_IS_LOCAL_ONLY` when
  `GITHUB_ACTIONS` is set.
* `ops/ci/workflow-policy.sh` refuses any hosted workflow that names the lane,
  so the job cannot be added to the graph by hand.

## Running it

    bash scripts/ci-local.sh devnode

It refuses, by name, when a prerequisite is missing rather than skipping quietly:
the Tuiwright checkout, the installed binaries, and a reachable daemon each have
their own typed refusal and remedy.

## What `devnode/tui` is

A black-box console harness. It drives the **installed** `bullet` binary through a
real pseudo-terminal and asserts on strings the console actually renders. It lives
here rather than in `bullet-kernel` because it tests a shipped binary from the
outside, and because it must not enter the Kernel's workspace or its lock.

Two things about this console that a future test author will otherwise rediscover
the hard way:

* **Never wait for an idle screen.** The console repaints its snapshot clock on a
  two-second poll, so it does not reach a quiet window and every idle wait times
  out. Assert on rendered text instead.
* **Run single-threaded.** Of six concurrent consoles, two never reached first
  paint within fifteen seconds; serialised, none failed. That is the whole of
  what has been measured. The cause is believed to be transient contention on the
  credential store's startup read lock, and the concurrency at which it begins
  has not been established, so the harness serialises rather than claiming a
  limit it has not proved.
