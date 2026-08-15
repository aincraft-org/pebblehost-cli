# PebbleHost CLI

A Rust command-line interface for the PebbleHost client API.

## Setup

```bash
cargo install --path .
export PEBBLEHOST_API_TOKEN=...
```

## Usage

```bash
pebblehost account
pebblehost servers
pebblehost server SERVER_ID
pebblehost power SERVER_ID --action start
pebblehost command SERVER_ID --command "say hello"
pebblehost --json servers
```

The API token is read from `PEBBLEHOST_API_TOKEN` by default. Use `--token` to override it, and `--base-url` to point at a different panel.

The implementation follows the published OpenAPI document at https://api.pebblehost.com/api.yaml and uses documented bearer-token authentication.
