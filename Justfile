# Bullet Farm hub command surface.

default:
    @just --list

setup:
    bash scripts/setup.sh

demo:
    bash scripts/demo.sh

fast:
    bash scripts/ci-local.sh fast

check:
    bash scripts/ci-local.sh required

verify: check

farmd:
    bash scripts/farmd.sh

portal:
    bash scripts/portal.sh
