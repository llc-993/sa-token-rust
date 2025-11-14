#!/usr/bin/env bash

set -euo pipefail

WORKSPACE_ROOT="/Users/m1pro/rustproject/sa-token-rust"
SLEEP_SECONDS="${SLEEP_SECONDS:-30}"

publish() {
  local manifest="$1"
  echo "Publishing ${manifest}..."
  cargo publish --manifest-path "$manifest"
  echo "Waiting ${SLEEP_SECONDS}s for crates.io index to update..."
  sleep "${SLEEP_SECONDS}"
}

publish "$WORKSPACE_ROOT/sa-token-adapter/Cargo.toml"
publish "$WORKSPACE_ROOT/sa-token-storage-memory/Cargo.toml"
publish "$WORKSPACE_ROOT/sa-token-storage-redis/Cargo.toml"
publish "$WORKSPACE_ROOT/sa-token-storage-database/Cargo.toml"
publish "$WORKSPACE_ROOT/sa-token-core/Cargo.toml"
publish "$WORKSPACE_ROOT/sa-token-macro/Cargo.toml"
publish "$WORKSPACE_ROOT/sa-token-plugin-actix-web/Cargo.toml"
publish "$WORKSPACE_ROOT/sa-token-plugin-axum/Cargo.toml"
publish "$WORKSPACE_ROOT/sa-token-plugin-gotham/Cargo.toml"
publish "$WORKSPACE_ROOT/sa-token-plugin-ntex/Cargo.toml"
publish "$WORKSPACE_ROOT/sa-token-plugin-poem/Cargo.toml"
publish "$WORKSPACE_ROOT/sa-token-plugin-rocket/Cargo.toml"
publish "$WORKSPACE_ROOT/sa-token-plugin-salvo/Cargo.toml"
publish "$WORKSPACE_ROOT/sa-token-plugin-tide/Cargo.toml"
publish "$WORKSPACE_ROOT/sa-token-plugin-warp/Cargo.toml"
publish "$WORKSPACE_ROOT/sa-token-rust/Cargo.toml"

echo "All crates published."

