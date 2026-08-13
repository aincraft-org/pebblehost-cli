from __future__ import annotations

import argparse
import json
import os
import sys
from .client import PebbleHostClient, PebbleHostError

def _parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="pebblehost", description="Manage PebbleHost servers from the command line")
    p.add_argument("--token", default=os.getenv("PEBBLEHOST_API_TOKEN"), help="API token (default: PEBBLEHOST_API_TOKEN)")
    p.add_argument("--base-url", default=os.getenv("PEBBLEHOST_BASE_URL", "https://panel.pebblehost.com"))
    p.add_argument("--json", action="store_true", dest="as_json", help="print JSON without human formatting")
    sub = p.add_subparsers(dest="command", required=True)
    sub.add_parser("account")
    sub.add_parser("servers")
    s = sub.add_parser("server"); s.add_argument("server_id")
    s = sub.add_parser("power"); s.add_argument("server_id"); s.add_argument("--action", choices=["start", "stop", "restart", "kill"], required=True)
    s = sub.add_parser("command"); s.add_argument("server_id"); s.add_argument("--command", dest="command_text", required=True)
    for name in ["resources", "activity", "backups", "databases", "allocations", "schedules", "players"]:
        parser = sub.add_parser(name); parser.add_argument("server_id")
    for name in ["plugins", "modpacks"]:
        parser = sub.add_parser(name); parser.add_argument("server_id"); parser.add_argument("--provider", required=True); parser.add_argument("--page", type=int, default=1); parser.add_argument("--page-size", type=int, default=20); parser.add_argument("--search-query")
    s = sub.add_parser("files"); s.add_argument("server_id"); s.add_argument("--directory", default="/")
    s = sub.add_parser("file-search"); s.add_argument("server_id"); s.add_argument("query"); s.add_argument("--root", default="/")
    s = sub.add_parser("file"); s.add_argument("server_id"); s.add_argument("path")
    return p

def _print(value, as_json: bool) -> None:
    if as_json or not isinstance(value, (dict, list)):
        print(json.dumps(value, indent=2, sort_keys=True) if isinstance(value, (dict, list)) else value)
    else:
        print(json.dumps(value, indent=2, sort_keys=True))

def main(argv: list[str] | None = None) -> int:
    args = _parser().parse_args(argv)
    if not args.token:
        print("error: provide --token or set PEBBLEHOST_API_TOKEN", file=sys.stderr)
        return 2
    try:
        with PebbleHostClient(args.token, args.base_url) as client:
            if args.command == "account":
                value = client.account()
            elif args.command == "servers":
                value = client.servers()
            elif args.command == "server":
                value = client.server(args.server_id)
            elif args.command == "power":
                value = client.power(args.server_id, args.action)
            elif args.command == "command":
                value = client.command(args.server_id, args.command_text)
            elif args.command in {"resources", "activity", "backups", "databases", "allocations", "schedules", "players"}:
                value = getattr(client, args.command)(args.server_id)
            elif args.command in {"plugins", "modpacks"}:
                value = getattr(client, args.command)(args.server_id, args.provider, args.page, args.page_size, args.search_query)
            elif args.command == "files":
                value = client.list_files(args.server_id, args.directory)
            elif args.command == "file-search":
                value = client.search_files(args.server_id, args.query, args.root)
            else:
                value = client.file_contents(args.server_id, args.path)
        _print(value, args.as_json)
        return 0
    except (PebbleHostError, ValueError) as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
