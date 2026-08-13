from __future__ import annotations

from typing import Any
import httpx

class PebbleHostError(RuntimeError):
    def __init__(self, status: int, message: str):
        super().__init__(f"PebbleHost API error ({status}): {message}")
        self.status = status
        self.message = message

class PebbleHostClient:
    def __init__(self, token: str, base_url: str = "https://panel.pebblehost.com", timeout: float = 30.0, transport: httpx.BaseTransport | None = None):
        if not token.strip():
            raise ValueError("API token must not be empty")
        self._client = httpx.Client(base_url=base_url.rstrip("/"), headers={"Authorization": f"Bearer {token}", "Accept": "application/json"}, timeout=timeout, transport=transport)

    def close(self) -> None:
        self._client.close()

    def __enter__(self):
        return self

    def __exit__(self, *_args):
        self.close()

    def request(self, method: str, path: str, *, params: dict[str, Any] | None = None, json: Any = None) -> Any:
        response = self._client.request(method, path, params=params, json=json)
        if response.is_error:
            try:
                detail = response.json()
            except ValueError:
                detail = response.text.strip() or response.reason_phrase
            raise PebbleHostError(response.status_code, str(detail))
        if not response.content:
            return None
        try:
            return response.json()
        except ValueError:
            return response.text

    def account(self): return self.request("GET", "/api/client/account")
    def servers(self): return self.request("GET", "/api/client")
    def server(self, server_id: str): return self.request("GET", f"/api/client/servers/{server_id}")
    def power(self, server_id: str, action: str): return self.request("POST", f"/api/client/servers/{server_id}/power", json={"signal": action})
    def command(self, server_id: str, command: str): return self.request("POST", f"/api/client/servers/{server_id}/command", json={"command": command})
    def resources(self, server_id: str): return self.request("GET", f"/api/client/servers/{server_id}/resources")
    def activity(self, server_id: str): return self.request("GET", f"/api/client/servers/{server_id}/activity")
    def backups(self, server_id: str): return self.request("GET", f"/api/client/servers/{server_id}/backups")
    def databases(self, server_id: str): return self.request("GET", f"/api/client/servers/{server_id}/databases")
    def allocations(self, server_id: str): return self.request("GET", f"/api/client/servers/{server_id}/network/allocations")
    def schedules(self, server_id: str): return self.request("GET", f"/api/client/servers/{server_id}/schedules")
    def players(self, server_id: str): return self.request("GET", f"/api/client/servers/{server_id}/minecraft/players")
    def plugins(self, server_id: str, provider: str, page: int = 1, page_size: int = 20, search_query: str | None = None): return self.request("GET", f"/api/client/servers/{server_id}/minecraft/plugins", params={"provider": provider, "page": page, "page_size": page_size, **({"search_query": search_query} if search_query else {})})
    def modpacks(self, server_id: str, provider: str, page: int = 1, page_size: int = 20, search_query: str | None = None): return self.request("GET", f"/api/client/servers/{server_id}/minecraft/modpacks", params={"provider": provider, "page": page, "page_size": page_size, **({"search_query": search_query} if search_query else {})})

    def list_files(self, server_id: str, directory: str = "/"):
        return self.request("GET", f"/api/client/servers/{server_id}/files/list", params={"directory": directory})

    def search_files(self, server_id: str, query: str, root: str = "/"):
        return self.request("GET", f"/api/client/servers/{server_id}/files/search", params={"root": root, "query": query})

    def file_contents(self, server_id: str, file: str):
        return self.request("GET", f"/api/client/servers/{server_id}/files/contents", params={"file": file})

# Expanded read-only domain methods are part of the CLI surface.
