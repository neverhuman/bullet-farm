# Bullet Farm

[neverhuman/bulletfarm](https://github.com/neverhuman/bulletfarm) is the public
aggregate Git repository. It contains four member source trees:

| Member | Role |
| --- | --- |
| [bullet-farm](bullet-farm/README.md) | Hub, installer, contracts, and assurance |
| [bullet-kernel](bullet-kernel/README.md) | Control plane and runtime boundaries |
| [bullet-git](bullet-git/README.md) | Candidate graph, journal, and proof roots |
| [bullet-portal](bullet-portal/README.md) | Operations portal |

Generated snapshots record each member's commit, tree, object format and source
ref in [`publication.json`](publication.json). Original source objects are retained
under `refs/tags/bullet-source/v1/<member>/<commit>`. An aggregate clone has one
Git checkout; its member directories are source trees, not independent checkouts.

Bullet is not release certified. The existing
[G1–G18 register](bullet-farm/docs/assurance/product-gaps.md) and
[full-product plan](bullet-farm/docs/assurance/full-product-dogfood-plan.md)
track the remaining work. Generated workflows derive jobs and matrix cells from
the reviewed member catalog; unavailable execution remains non-passing.
Installed operation, authenticated provider tasks, independent verification and
approved integration require their own evidence. Operating HOLD remains.

## Clone and build from source

Use Linux, Git, Node 22.23.2 and npm 10.9.8. Install both checked-in Rust
toolchains: 1.95.0 for the Hub and 1.97.1 for Kernel. Rustup selects the
member's `rust-toolchain.toml` when running its commands:

```bash
git clone https://github.com/neverhuman/bulletfarm.git
cd bulletfarm
git rev-parse HEAD

(cd bullet-kernel && cargo build --locked -p bullet --bin bullet \
  -p bullet-farmd --bin bullet-farmd)
(cd bullet-portal && npm ci && npm run build)
```

Record the clone commit with any build report. These commands compile source;
they do not install a persistent service or establish release qualification.
Clean-host builds and lifecycle acceptance remain required. The
[source setup guide](bullet-farm/docs/runbooks/source-setup.md) explains the
separate canonical family layout and the currently blocked `just setup` path.
A source archive or shallow clone does not carry the full Git history needed
for canonical proof or exact publication verification.

For the unsigned local console, install Just and the additional tools listed in
the [contributor guide](bullet-farm/CONTRIBUTING.md), then from `bullet-farm/`:

```bash
just -- console --data-dir "$HOME/.local/state/bullet-operator-console"
```

This keeps the console in the foreground and uses the selected state directory.
For a new local instance, choose a new directory under your home, outside the
clone and outside `/tmp`.
The launcher prints access instructions and refuses unsupported prerequisites.
Its component checks do not qualify installed service or provider operation.
`just preview` and canonical proof lanes require independent member checkouts;
they are not aggregate-clone acceptance commands.

## Contributions, security and licensing

Open source changes against the member repository that owns the files; see
[Contributing](bullet-farm/CONTRIBUTING.md). Root files are generated from the
Hub's `publication/root/` templates and `publication/config.json`. Submit root
changes there so reviewed generation preserves exact member trees. Human review
and approved publication remain required; a direct aggregate edit fails
regeneration checks.

Follow the [security reporting guidance](bullet-farm/SECURITY.md) before sharing
vulnerability details. Private reporting was disabled on all five repositories
when checked on 2026-09-11; request a private contact without public details.

For setup questions or bug reports, see [Support](SUPPORT.md). The issue forms
ask for a reproduction and the exact source or installed version.

The aggregate uses [Apache-2.0](LICENSE); retain the member notices and
[third-party notices](NOTICE), including the Kernel qualification assets' MIT
and OFL licenses. Public-release acceptance remains pending.

## Recording evidence

Retained operator-console diagnostics (local loopback observation;
not installed or provider qualification; Operating HOLD remains):

- [operator TUI](bullet-farm/media/operator-console/operator-tui.gif)
- [operator Portal](bullet-farm/media/operator-console/operator-portal.gif)
- [captions](bullet-farm/media/operator-console/README.md)

Historical recordings remain available in the source snapshot and are not the
current operator showcase. The three retained bridge GIFs do not meet the
required native capture geometry.

- [Contained Claude Candidate recording](bullet-farm/media/dogfood/candidate/dogfood-candidate-hi.gif)
  and [reproduction record](bullet-farm/media/dogfood/README.md): a retained
  component bridge with a preserved Candidate and cleanup refusal. It does not
  establish installed Bullet authentication, TUI/Portal control of the same
  task, independent verification, protected integration or authoritative read-back.
- [Earlier vendor-CLI and Portal recordings](bullet-farm/docs/demo-gif/README.md):
  vendor explanation sessions and a Portal form demonstration, with their
  original limitations. They do not establish the requested provider workflow.
The installed TUI and packaged Portal must still demonstrate the same real
task/run/Attempt/Candidate/review/integration identities, with authenticated
provider execution and preserved original timing. Follow the existing
[xbabe2 capture specification](bullet-farm/docs/assurance/xbabe2-development-closeout.md).
No three-provider or four-provider milestone is claimed here.
