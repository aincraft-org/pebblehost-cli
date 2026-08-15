# PebbleHost CLI

An unofficial Rust command-line interface for the PebbleHost client API.

> Unofficial project: this CLI is not affiliated with, endorsed by, sponsored by, or otherwise associated with the PebbleHost brand.

## API coverage

The CLI includes convenient commands for common operations and an escape hatch for the complete published API:

```bash
# List the 141 operations in the bundled API inventory
pebblehost operations

# Call any documented endpoint directly
pebblehost api-call GET /api/client/servers/SERVER_ID/resources
pebblehost api-call POST /api/client/servers/SERVER_ID/command \
  --body '{"command":"say hello"}'
```

`api-call` accepts `GET`, `POST`, `PUT`, `PATCH`, and `DELETE`, repeatable `--query KEY=VALUE` parameters, and a raw JSON `--body`. The default request base URL follows the published OpenAPI server, `https://panel.pebblehost.com`; override it with `--base-url`.

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
