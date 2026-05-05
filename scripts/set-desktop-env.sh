#!/bin/env bash
# Sets required environment variables for a desktop (Tauri) production build.
# Source this file before running the mix release command.

export DATABASE_PATH="${DATABASE_PATH:-$HOME/.local/share/qitto/qitto.db}"

# Cache the generated secret so sessions survive rebuilds. Do NOT commit .secret_key_base.
SECRET_FILE="$(dirname "$0")/../.secret_key_base"
if [ ! -f "$SECRET_FILE" ]; then
  mix phx.gen.secret > "$SECRET_FILE"
fi
export SECRET_KEY_BASE="${SECRET_KEY_BASE:-$(cat "$SECRET_FILE")}"
