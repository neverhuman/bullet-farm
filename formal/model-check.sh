#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FORMAL="$ROOT/formal"
LOCK="$FORMAL/toolchain.lock.json"
MODEL_LOCK="$FORMAL/model-lock.json"
CACHE="$ROOT/target/tlc/v1.7.4"
JAR="${TLA2TOOLS_JAR:-$CACHE/tla2tools.jar}"
mkdir -p "$CACHE"

for required in java jq sha256sum curl; do
  command -v "$required" >/dev/null 2>&1 || {
    echo "formal-check: missing required tool $required" >&2
    exit 2
  }
done

expected_sha="$(jq -er '.tlc_jar_sha256' "$LOCK")"
expected_sha1="$(jq -er '.tlc_jar_sha1' "$LOCK")"
url="$(jq -er '.url' "$LOCK")"
expected_java="$(jq -er '.java_major' "$LOCK")"

if [[ ! -f "$JAR" ]]; then
  [[ -z "${TLA2TOOLS_JAR:-}" ]] || {
    echo "formal-check: explicit TLA2TOOLS_JAR does not exist: $JAR" >&2
    exit 2
  }
  partial="$CACHE/tla2tools.jar.partial"
  curl --proto '=https' --tlsv1.2 -fLSs "$url" -o "$partial"
  echo "$expected_sha  $partial" | sha256sum --check --status || {
    echo "formal-check: downloaded TLC jar failed SHA-256 pin" >&2
    exit 2
  }
  mv "$partial" "$JAR"
fi

echo "$expected_sha  $JAR" | sha256sum --check --status || {
  echo "formal-check: TLC jar SHA-256 does not match toolchain lock" >&2
  exit 2
}
actual_sha1="$(sha1sum "$JAR" | awk '{print $1}')"
[[ "$actual_sha1" == "$expected_sha1" ]] || {
  echo "formal-check: TLC jar SHA-1 does not match upstream release checksum" >&2
  exit 2
}
java_major="$(java -version 2>&1 | sed -nE '1s/.*version "([0-9]+).*/\1/p')"
[[ "$java_major" == "$expected_java" ]] || {
  echo "formal-check: Java major $java_major does not match pinned major $expected_java" >&2
  exit 2
}

mapfile -t modules < <(find "$FORMAL" -maxdepth 1 -type f -name '*.tla' -printf '%f\n' | sort)
[[ "${#modules[@]}" -eq 2 ]] || {
  echo "formal-check: exactly two TLA+ modules are permitted; found ${#modules[@]}" >&2
  exit 2
}

for module in "${modules[@]}"; do
  name="${module%.tla}"
  config="$name.cfg"
  expected_module="$(jq -er --arg module "$module" '.models[] | select(.module == $module) | .module_sha256' "$MODEL_LOCK")"
  expected_config="$(jq -er --arg module "$module" '.models[] | select(.module == $module) | .config_sha256' "$MODEL_LOCK")"
  expected_generated="$(jq -er --arg module "$module" '.models[] | select(.module == $module) | .generated_states' "$MODEL_LOCK")"
  expected_distinct="$(jq -er --arg module "$module" '.models[] | select(.module == $module) | .distinct_states' "$MODEL_LOCK")"
  echo "$expected_module  $FORMAL/$module" | sha256sum --check --status || {
    echo "formal-check: $module differs from model lock" >&2
    exit 2
  }
  echo "$expected_config  $FORMAL/$config" | sha256sum --check --status || {
    echo "formal-check: $config differs from model lock" >&2
    exit 2
  }
  log="$CACHE/$name.log"
  (
    cd "$FORMAL"
    java -XX:+UseParallelGC -cp "$JAR" tlc2.TLC -cleanup -workers 1 \
      -seed 20260824 -fp 0 -config "$config" "$module"
  ) | tee "$log"
  grep -Fq "Model checking completed. No error has been found." "$log" || {
    echo "formal-check: $name did not complete cleanly" >&2
    exit 2
  }
  counts="$(sed -nE 's/^([0-9]+) states generated, ([0-9]+) distinct states found.*/\1 \2/p' "$log")"
  read -r generated distinct <<< "$counts"
  [[ "$generated" == "$expected_generated" && "$distinct" == "$expected_distinct" ]] || {
    echo "formal-check: $name states drifted: generated=$generated distinct=$distinct" >&2
    exit 2
  }
done

echo "formal-check: 2/2 models match pinned tool, source, config, and state counts"
