# justdeploy

Deploy projects to remote servers without leaving your terminal.

justdeploy is a CLI tool with a built-in TUI that handles the entire deployment workflow over SSH — register your projects and servers once, then deploy with a single command.

## Quick Start

```bash
# Install (from source)
cd cli && cargo build --release
cp target/release/justdeploy /usr/local/bin/

# Register a project
jd project add ./my-app

# Register a server
jd server add prod --ip 203.0.113.10 --key ~/.ssh/id_ed25519

# Deploy
jd deploy my-app --to prod
```

## How It Works

1. **Add a project** — point justdeploy at a directory containing a `jd.json` config or a `Dockerfile`.
2. **Add a server** — register a remote server with password or SSH key authentication.
3. **Deploy** — justdeploy connects over SSH, builds, transfers, and runs your project on the target server.

## Configuration

### `jd.json`

Place a `jd.json` in your project root to define how it should be built and deployed:

```json
{
  "name": "my-app",
  "build": "cargo build --release",
  "run": "./target/release/my-app",
  "ports": [8080],
  "env": {
    "RUST_LOG": "info"
  }
}
```

### Dockerfile

Alternatively, just include a `Dockerfile` in your project root. justdeploy will build and run the container on the remote server.

## Usage

```
jd [COMMAND]
```

Running `jd` with no arguments launches the interactive TUI.

### Commands

| Command | Description |
|---|---|
| `jd project add <path>` | Register a project directory |
| `jd project list` | List all registered projects |
| `jd project remove <name>` | Unregister a project |
| `jd server add <name> --ip <ip> [--key <path> \| --password]` | Register a remote server |
| `jd server list` | List all registered servers |
| `jd server remove <name>` | Unregister a server |
| `jd deploy <project> --to <server>` | Deploy a project to a server |

## Authentication

justdeploy supports two SSH authentication methods:

- **SSH key** (recommended) — `jd server add prod --ip 203.0.113.10 --key ~/.ssh/id_ed25519`
- **Password** — `jd server add prod --ip 203.0.113.10 --password` (prompts for password)

## Project Structure

```
justdeploy/
├── cli/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs            # Entry point, CLI arg parsing
│       ├── app.rs             # TUI application state and event loop
│       ├── config.rs          # Global config (~/.justdeploy/)
│       ├── error.rs           # Error types
│       ├── commands/          # CLI subcommand handlers
│       ├── ui/                # TUI screens and widgets (ratatui)
│       ├── models/            # Data structures (project, server, deploy)
│       ├── ssh/               # SSH connectivity layer
│       └── deploy/            # Deployment orchestration and strategies
└── README.md
```

## Built With

- [Rust](https://www.rust-lang.org/)
- [ratatui](https://github.com/ratatui/ratatui) — terminal UI
- [clap](https://github.com/clap-rs/clap) — CLI argument parsing
- [ssh2](https://github.com/alexcrichton/ssh2-rs) — SSH connections

## Status

Early stage — under active development.

## License

MIT
