import httpx
from pebblehost_cli import cli

def test_cli_requires_token(capsys):
    assert cli.main(["servers"]) == 2
    assert "PEBBLEHOST_API_TOKEN" in capsys.readouterr().err

def test_cli_json(monkeypatch, capsys):
    class Fake:
        def __init__(self, token, base_url): pass
        def __enter__(self): return self
        def __exit__(self, *args): pass
        def servers(self): return {"data": [1]}
    monkeypatch.setattr(cli, "PebbleHostClient", Fake)
    assert cli.main(["--token", "secret", "--json", "servers"]) == 0
    assert '"data"' in capsys.readouterr().out


def test_file_search_cli_uses_root(monkeypatch):
    seen = {}
    class Fake:
        def __init__(self, token, base_url): pass
        def __enter__(self): return self
        def __exit__(self, *args): pass
        def search_files(self, server_id, query, root):
            seen.update(server_id=server_id, query=query, root=root)
            return {"data": []}
    monkeypatch.setattr(cli, "PebbleHostClient", Fake)
    assert cli.main(["--token", "secret", "file-search", "srv-1", "paper", "--root", "/plugins"]) == 0
    assert seen == {"server_id": "srv-1", "query": "paper", "root": "/plugins"}
