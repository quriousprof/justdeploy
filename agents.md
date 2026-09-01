# justdeploy

A CLI tool for managing deployments directly from your terminal.

## What it does

justdeploy lets you deploy projects to remote servers without leaving the terminal. The core workflow:

1. **Add a project** — point justdeploy at a directory. It uses a `jd.json` config file to understand how to build and deploy, or you can provide a Dockerfile directly.
2. **Add servers** — register server IPs with password or SSH key authentication.
3. **Deploy** — justdeploy SSHs into the target server(s) and handles the deployment.

## Tech stack

- **Language:** Rust
- **TUI:** ratatui (terminal UI framework)
- **CLI location:** `cli/` directory

## Project structure

```
justdeploy/
├── cli/                  # Rust CLI application
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── agents.md             # This file
```

## Key concepts

- **`jd.json`** — per-project config file that lives in the project directory. Defines how the project should be built and deployed.
- **Dockerfile support** — projects can alternatively provide a Dockerfile instead of a `jd.json` for containerized deployments.
- **Server management** — users register remote servers (IP + password or SSH key) that justdeploy can deploy to.
- **SSH-based deployment** — deployments happen over SSH to the registered servers.

## Status

Early stage — scaffolding only. The CLI binary exists but core functionality is not yet implemented.
