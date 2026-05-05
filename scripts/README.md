# Desktop Release Environment Setup

Before running `cargo tauri build`, two environment variables must be set.

---

## Variables

| Variable | Required | Description |
|---|---|---|
| `DATABASE_PATH` | Yes | Absolute path to the SQLite database file |
| `SECRET_KEY_BASE` | Yes | 64-character secret used to sign cookies/sessions |

---

## Option A — Manual export (one-time per terminal session)

```bash
export DATABASE_PATH="$HOME/.local/share/qitto/qitto.db"
export SECRET_KEY_BASE="$(mix phx.gen.secret)"
```

Then build:

```bash
cargo tauri build
```

> `mix phx.gen.secret` generates a new random secret each time.
> If you want sessions to survive rebuilds, save the output and reuse the same value.

---

## Option B — Source the helper script

```bash
source scripts/set-desktop-env.sh
cargo tauri build
```

The script sets `DATABASE_PATH` to the default path above and generates
`SECRET_KEY_BASE` once, caching it in `.secret_key_base` (gitignored) so the
same key is reused on future builds.

---

## Verify the variables are set

```bash
echo $DATABASE_PATH
echo $SECRET_KEY_BASE
```

Both must be non-empty before running the build.

---

## Kill stale Erlang processes before rebuilding

If a previous `cargo tauri dev` session is still running, the build will fail
with **"Text file busy"**. Kill the processes first:

```bash
pkill -f "beam.smp" || true
pkill -f "epmd" || true
```
