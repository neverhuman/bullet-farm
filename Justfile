# Bullet Farm hub command surface.

default:
    @just --list

setup:
    bash scripts/setup.sh

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

security:
    bash scripts/ci-local.sh security

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

lock-check tag:
    cargo run --locked --quiet --bin bullet-family -- lock check --tag {{tag}}
