#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

ENV_ARGS=()
if [ -f "$REPO_ROOT/.env" ]; then
  ENV_ARGS=(--env-file "$REPO_ROOT/.env")
fi

GENERATOR_SOURCE="meshy-content-generator==0.2.0"
if [ -n "${MESHY_CONTENT_GENERATOR_SOURCE:-}" ]; then
  GENERATOR_SOURCE="$MESHY_CONTENT_GENERATOR_SOURCE"
fi

uvx "${ENV_ARGS[@]}" \
  --from "$GENERATOR_SOURCE" \
  content-gen run "$SCRIPT_DIR/pipeline.json" --root "$REPO_ROOT" "$@"
