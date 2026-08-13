import httpx
import json
import pytest
from pebblehost_cli.client import PebbleHostClient, PebbleHostError

def test_client_uses_bearer_and_decodes_json():
    def handler(request):
        assert request.headers["Authorization"] == "Bearer secret"
        assert request.url.path == "/api/client"
        return httpx.Response(200, json={"data": []})
    with PebbleHostClient("secret", "https://example.test", transport=httpx.MockTransport(handler)) as client:
        assert client.servers() == {"data": []}

def test_client_reports_api_errors():
    def handler(request): return httpx.Response(401, json={"errors": [{"detail": "Unauthenticated"}]})
    with PebbleHostClient("secret", transport=httpx.MockTransport(handler)) as client:
        with pytest.raises(PebbleHostError) as error: client.account()
        assert error.value.status == 401
        assert "Unauthenticated" in str(error.value)


def test_command_sends_user_command_body():
    def handler(request):
        assert request.method == "POST"
        assert request.url.path == "/api/client/servers/srv-1/command"
        assert json.loads(request.content) == {"command": "say hello"}
        return httpx.Response(200, json={"ok": True})
    with PebbleHostClient("secret", transport=httpx.MockTransport(handler)) as client:
        assert client.command("srv-1", "say hello") == {"ok": True}


def test_resources_path():
    def handler(request):
        assert request.url.path == "/api/client/servers/srv-1/resources"
        return httpx.Response(200, json={"resources": {}})
    with PebbleHostClient("secret", transport=httpx.MockTransport(handler)) as client:
        assert client.resources("srv-1") == {"resources": {}}

def test_cli_command_forwards_command_text(monkeypatch):
    seen = {}
    class Fake:
        def __init__(self, token, base_url): pass
        def __enter__(self): return self
        def __exit__(self, *args): pass
        def command(self, server_id, command): seen.update(server_id=server_id, command=command); return {"ok": True}
    monkeypatch.setattr("pebblehost_cli.cli.PebbleHostClient", Fake)
    from pebblehost_cli.cli import main
    assert main(["--token", "secret", "command", "srv-1", "--command", "say hello"]) == 0
    assert seen == {"server_id": "srv-1", "command": "say hello"}


def test_plugins_sends_documented_query():
    def handler(request):
        assert request.url.path == "/api/client/servers/srv-1/minecraft/plugins"
        assert dict(request.url.params) == {"provider": "modrinth", "page": "2", "page_size": "10", "search_query": "worldedit"}
        return httpx.Response(200, json={"data": []})
    with PebbleHostClient("secret", transport=httpx.MockTransport(handler)) as client:
        assert client.plugins("srv-1", "modrinth", 2, 10, "worldedit") == {"data": []}


def test_search_files_sends_documented_parameters():
    def handler(request):
        assert request.url.path == "/api/client/servers/srv-1/files/search"
        assert dict(request.url.params) == {"root": "/plugins", "query": "paper"}
        return httpx.Response(200, json={"data": []})
    with PebbleHostClient("secret", transport=httpx.MockTransport(handler)) as client:
        assert client.search_files("srv-1", "paper", "/plugins") == {"data": []}
