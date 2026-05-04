# GhostKey

**Local-first secrets manager for developers.**

Stop using `.env` files. Manage secrets visually, use them programmatically.

[Features](#features) -- [Quick Start](#quick-start) -- [CLI](#cli-commands) -- [Web UI](#web-ui) -- [Security](#security)

---

## Why GhostKey?

| Problem | GhostKey Solution |
|---------|-------------------|
| `.env` files in git | Encrypted vault, never in plaintext |
| Secrets scattered across tools | One place for all API keys, tokens, passwords |
| Hard to share env setup | `ghostkey run -- npm start` injects everything |
| No visual overview | Clean web UI for managing secrets |

## Features

- **Web UI** -- Clean, minimal interface for managing secrets
- **CLI** -- 5 commands, not 15. Simple and focused.
- **Encrypted vault** -- AES-256-GCM + Argon2id. Zero plaintext.
- **Project/Environment model** -- Organize by project, switch between dev/staging/prod
- **`ghostkey run`** -- Inject secrets as env vars into any command
- **Local-first** -- No cloud, no accounts, no tracking

## Quick Start

### Install

```bash
# From source
git clone https://github.com/yourusername/ghostkey.git
cd ghostkey
cargo install --path .
```

### Initialize

```bash
ghostkey init
```

### Start the Web UI

```bash
ghostkey dev
```

Opens http://localhost:PORT in your browser. Create projects, add secrets, manage environments.

### Run commands with secrets

```bash
ghostkey run -- npm start
ghostkey run --project myapp --env staging -- python server.py
```

### Export for CI/CD

```bash
ghostkey export --format env > .env
ghostkey export --format json --output backup.json
```

## CLI Commands

| Command | Description |
|---------|-------------|
| `ghostkey init` | Create vault and set master password |
| `ghostkey dev` | Start API server and open web UI |
| `ghostkey run -- <cmd>` | Run command with secrets as env vars |
| `ghostkey export` | Export secrets to file or stdout |
| `ghostkey import` | Import secrets from file |

### `ghostkey run`

The primary workflow command. Replaces `.env` files entirely.

```bash
# Run with default project/environment
ghostkey run -- npm start

# Specify project and environment
ghostkey run --project myapp --env production -- ./deploy.sh

# Works with any command
ghostkey run -- docker compose up
ghostkey run -- python manage.py migrate
```

### `ghostkey export`

```bash
# Export as .env format (default)
ghostkey export

# Export as JSON
ghostkey export --format json

# Export specific environment
ghostkey export --project myapp --env staging --output staging.env
```

### `ghostkey import`

```bash
# Import from .env file
ghostkey import --format env .env

# Import into specific environment
ghostkey import --format env --project myapp --env production prod.env
```

## Web UI

The web UI provides a visual interface for managing secrets:

- **Dashboard** -- List all projects with secret counts
- **Project view** -- Switch between environments (dev/staging/prod)
- **Secret table** -- View, add, edit, delete secrets with mask/unmask
- **Search** -- Filter secrets by key or description

The UI follows a minimal white design (Linear/Vercel aesthetic):

- White background, subtle borders
- Clean typography (Inter font)
- Blue accent (#2563EB) for primary actions
- No enterprise dashboard clutter

## Architecture

```
ghostkey (Rust binary)
├── CLI layer (5 commands)
├── API server (axum, localhost only)
├── Vault core (AES-256-GCM + Argon2id)
└── Storage (single encrypted file at ~/.ghostkey/vault.enc)

web/ (Next.js frontend)
├── Dashboard (project list)
├── Project page (env switcher + secret table)
└── Unlock page (master password entry)
```

## Data Model

```
Project
├── Environment (dev, staging, production)
│   ├── Secret (KEY=value)
│   ├── Secret (KEY=value)
│   └── ...
└── Environment (production)
    └── ...
```

Each secret is individually encrypted. The entire vault is one encrypted file.

## Security

- **Encryption**: AES-256-GCM (authenticated encryption)
- **Key derivation**: Argon2id (64MB memory, 3 iterations, 4 threads)
- **Zero plaintext**: Secrets are never stored unencrypted
- **Local only**: API server binds to 127.0.0.1, no remote access
- **No auth tokens**: Master password unlocks vault for the session

### Threat Model

| Threat | Protection |
|--------|------------|
| File access | AES-256-GCM encryption |
| Git leak | .gitignore excludes vault |
| Stolen device | Full disk encryption recommended |
| Memory dump | Rust memory safety |

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Start API server
cargo run -- dev

# Start web UI (separate terminal)
cd web && npm run dev

# Makefile shortcuts
make dev        # Start API + open browser
make dev-api    # Start API only
make dev-web    # Start web UI only
make test       # Run all tests
```

## Project Structure

```
ghostkey/
├── src/
│   ├── api/           # HTTP API (axum)
│   │   ├── handlers.rs
│   │   ├── routes.rs
│   │   └── types.rs
│   ├── cli/           # CLI commands (5 total)
│   │   ├── init.rs
│   │   ├── dev.rs
│   │   ├── run.rs
│   │   ├── export.rs
│   │   └── import.rs
│   ├── vault/         # Core encryption + storage
│   │   ├── crypto.rs  # AES-256-GCM
│   │   ├── key.rs     # Argon2id
│   │   ├── storage.rs # Vault file I/O
│   │   └── migration.rs
│   ├── models/        # Data models
│   │   ├── project.rs
│   │   ├── environment.rs
│   │   └── secret.rs
│   └── main.rs
├── web/               # Next.js frontend
│   └── src/
│       ├── app/       # Pages
│       ├── components/
│       └── lib/       # API client, types
├── Cargo.toml
└── Makefile
```

## License

MIT OR Apache-2.0
