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
cd src-tauri
cargo tauri build
```

That single command does everything automatically:

1. Runs `MIX_ENV=prod mix do compile + assets.deploy + release` — compiles Elixir, minifies assets, and builds a self-contained OTP release into `src-tauri/target/rel/`
2. Compiles the Rust binary
3. Bundles the Elixir release + Rust binary into a native installer

Distributable installers are output to `src-tauri/target/release/bundle/`:

| Platform | Output formats |
|---|---|
| Linux | `.deb`, `.AppImage` |
| macOS | `.dmg`, `.app` |
| Windows | `.msi`, `.exe` |

### Code signing (required for public distribution)

**macOS** — requires a paid Apple Developer account:

```bash
APPLE_SIGNING_IDENTITY="Developer ID Application: ..."  \
APPLE_ID="you@example.com"                               \
APPLE_PASSWORD="app-specific-password"                   \
APPLE_TEAM_ID="XXXXXXXXXX"                               \
cargo tauri build
```

**Windows** — uses Azure Trusted Signing. See the [Tauri Windows signing guide](https://v2.tauri.app/distribute/sign/windows/).

**Linux** — no signing needed; `.deb` and `.AppImage` can be distributed directly.

### CI/CD (GitHub Actions)

Use [`tauri-apps/tauri-action`](https://github.com/tauri-apps/tauri-action) to build for all platforms in parallel:

```yaml
strategy:
  matrix:
    include:
      - platform: macos-15
        target: aarch64-apple-darwin
      - platform: macos-15
        target: x86_64-apple-darwin
      - platform: ubuntu-22.04
        target: x86_64-unknown-linux-gnu
      - platform: windows-2022
        target: x86_64-pc-windows-msvc

runs-on: ${{ matrix.platform }}
steps:
  - uses: erlef/setup-beam@v1
    with:
      otp-version: "27"
      elixir-version: "1.17"

  - uses: tauri-apps/tauri-action@v0.6
    env:
      GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      MIX_ENV: prod
      # macOS codesigning
      APPLE_SIGNING_IDENTITY: ${{ secrets.APPLE_SIGNING_IDENTITY }}
      APPLE_ID: ${{ secrets.APPLE_ID }}
      APPLE_PASSWORD: ${{ secrets.APPLE_PASSWORD }}
      APPLE_TEAM_ID: ${{ secrets.APPLE_TEAM_ID }}
      # Windows codesigning
      AZURE_CLIENT_ID: ${{ secrets.AZURE_CLIENT_ID }}
      AZURE_CLIENT_SECRET: ${{ secrets.AZURE_CLIENT_SECRET }}
      AZURE_TENANT_ID: ${{ secrets.AZURE_TENANT_ID }}
```

> **Linux note**: `erlef/setup-beam` on Ubuntu dynamically links OpenSSL, so the release may not run on other distros. Use AppImage or build per-distro for broad Linux support.

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
