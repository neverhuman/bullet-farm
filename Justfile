# Bullet Farm hub command surface.

default:
    @just --list

[positional-arguments]
setup *args:
    exec bash scripts/setup.sh "$@"

coord *args:
    cargo run --locked --quiet --bin bullet-family -- coord {{args}}

demo:
    bash scripts/demo.sh

fast:
    bash scripts/ci-local.sh fast

check:
    bash scripts/ci-local.sh required

contract:
    bash scripts/ci-local.sh contract

family-contract:
    bash scripts/ci-local.sh family-contract

contract-generate:
    cargo run --locked --quiet -p bullet-wire --bin bullet-contract -- generate --root .

contract-check:
    cargo run --locked --quiet -p bullet-wire --bin bullet-contract -- check --root .

model-check:
    bash formal/model-check.sh

security:
    bash scripts/ci-local.sh security

audit:
    bash scripts/ci-local.sh audit

ci-doctor lane="all":
    bash scripts/ci-doctor.sh {{lane}}

hooks-install:
    git config --local core.hooksPath ops/git-hooks

family:
    bash scripts/ci-local.sh family

verify: check

farmd:
    bash scripts/farmd.sh

portal:
    bash scripts/portal.sh

check-family:
    bash scripts/ci-local.sh family

lock-generate tag:
    cargo run --locked --quiet --bin bullet-family -- lock generate --tag {{tag}}

lock-verify tag:
    cargo run --locked --quiet --bin bullet-family -- lock verify --tag {{tag}}
