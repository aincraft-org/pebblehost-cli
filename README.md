# PebbleHost CLI

A small Python CLI wrapper for the PebbleHost client API.

## Setup

```bash
python -m venv .venv
. .venv/bin/activate
pip install -e .
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

The token is read from `PEBBLEHOST_API_TOKEN` by default. `--token` overrides it.

The implementation follows the published OpenAPI document at https://api.pebblehost.com/api.yaml and uses the documented bearer-token authentication.

## Rust CLI

The Rust port is built with Cargo:

```bash
cargo install --path .
export PEBBLEHOST_API_TOKEN=...
pebblehost servers
```
