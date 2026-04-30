# Qitto

A desktop application built with **Phoenix LiveView** + **Tauri**. The backend is a full Phoenix web server (with SQLite) embedded inside a native desktop window via Tauri.

## Tech Stack

| Layer | Technology |
|---|---|
| UI | [Phoenix LiveView](https://hexdocs.pm/phoenix_live_view) + [Tailwind CSS v4](https://tailwindcss.com) |
| Backend | [Phoenix](https://www.phoenixframework.org/) + [Ecto](https://hexdocs.pm/ecto) |
| Database | [SQLite](https://www.sqlite.org/) via [ecto_sqlite3](https://github.com/elixir-sqlite/ecto_sqlite3) |
| Desktop shell | [Tauri v2](https://tauri.app/) (Rust) |
| Bridge | [ElixirKit](https://github.com/livebook-dev/elixirkit) — connects Tauri ↔ Phoenix over TCP PubSub |

## Prerequisites

- [Elixir](https://elixir-lang.org/install.html) >= 1.15
- [Rust](https://rustup.rs/) (stable toolchain)
- [Node.js](https://nodejs.org/) (for asset bundling via esbuild/tailwind)
- On Linux: `webkit2gtk`, `libssl-dev`, `libayatana-appindicator3-dev` (Tauri system deps — see [Tauri Linux prerequisites](https://tauri.app/start/prerequisites/))

## Development

```bash
# Install Elixir dependencies and set up the database
mix setup

# Start Phoenix server only (no desktop window)
mix phx.server

# Start the full desktop app (Tauri + Phoenix)
cd src-tauri
cargo tauri dev
```

The desktop app starts Tauri, which spawns `mix phx.server` as a child process and waits for Phoenix to broadcast `"ready"` before opening the window.

Visit [http://localhost:4000](http://localhost:4000) for browser-only development.

## Database

- **Dev**: SQLite file at `qitto_dev.db` in the project root
- **Production (bundled app)**: SQLite file at the OS app data directory
  - Linux: `~/.local/share/qitto/qitto.db`
  - macOS: `~/Library/Application Support/qitto/qitto.db`
  - Windows: `%APPDATA%\qitto\qitto.db`

Migrations run **automatically on every app startup** — no manual migration step needed after installing a new version.

```bash
# Generate a new migration
mix ecto.gen.migration migration_name

# Reset the dev database
mix ecto.reset
```

## Building for Production

```bash
# Build the Elixir release first
MIX_ENV=prod mix release

# Then build the Tauri bundle (includes the release)
cd src-tauri
cargo tauri build
```

Distributable installers are output to `src-tauri/target/release/bundle/`.

## Running Tests

```bash
mix test

# Run only previously failed tests
mix test --failed
```

## Pre-commit

```bash
mix precommit
```

Runs `compile --warnings-as-errors`, removes unused deps, formats code, and runs the full test suite.

## Project Structure

```
lib/
  qitto/          # Business logic, Ecto schemas, contexts
  qitto_web/      # Phoenix controllers, LiveViews, components
src-tauri/
  src/lib.rs      # Tauri setup — spawns Phoenix, manages the window
  Cargo.toml
  tauri.conf.json
priv/repo/
  migrations/     # Ecto migrations (auto-run on startup)
assets/
  css/app.css     # Tailwind v4 entry point
  js/app.js       # Phoenix LiveView + hooks
```

## Resources

- [Phoenix Framework](https://www.phoenixframework.org/)
- [Phoenix LiveView docs](https://hexdocs.pm/phoenix_live_view)
- [Tauri v2 docs](https://tauri.app/)
- [ElixirKit (Tauri ↔ Elixir bridge)](https://github.com/livebook-dev/elixirkit)
- [ecto_sqlite3](https://github.com/elixir-sqlite/ecto_sqlite3)
- [Tailwind CSS v4](https://tailwindcss.com/docs)
- [Elixir Forum](https://elixirforum.com/c/phoenix-forum)
