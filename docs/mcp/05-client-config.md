---
status: DRAFT
---

# MCP 05 — Client Configuration

How a user registers Smith's MCP server in each supported client. This document is the
normative source for Smith's own setup UI and documentation.

See [research/02-mcp-transport.md](../research/02-mcp-transport.md) §6 for the verified
client config formats.

## Server endpoint

Smith serves a single MCP endpoint:

```
http://127.0.0.1:<port>/mcp
```

- `<port>` is announced in the discovery file (see
  [mcp/01-server-design.md](./01-server-design.md) §Discovery) and shown in Smith's UI.
- Transport: Streamable HTTP, legacy era (protocol versions `2025-06-18` / `2025-11-25`).
- Auth: optional bearer token (configured in Smith settings); if set, clients send
  `Authorization: Bearer <token>`.

`[REQ-MCP-CFG-001]` Smith's UI MUST display, on demand:
- The full endpoint URL.
- The current bearer token (if auth is enabled), with a copy button.
- Ready-to-paste config snippets for each supported client (below).

## Claude Code

Docs: https://code.claude.com/docs/en/mcp

### CLI registration
```bash
claude mcp add --transport http smith http://127.0.0.1:<port>/mcp
# with auth:
claude mcp add --transport http smith http://127.0.0.1:<port>/mcp \
  --header "Authorization: Bearer <token>"
```

### Project-scoped `.mcp.json` (checked into project root)
```json
{
  "mcpServers": {
    "smith": {
      "type": "http",
      "url": "http://127.0.0.1:<port>/mcp",
      "headers": { "Authorization": "Bearer <token>" },
      "timeout": 600000
    }
  }
}
```

`[REQ-MCP-CFG-002]` The `type` field MUST be `"http"` (or alias `"streamable-http"`). Omitting
`type` defaults Claude Code to stdio interpretation, which fails.

## Codex (OpenAI)

Docs: https://learn.chatgpt.com/docs/config-file/config-reference (`~/.codex/config.toml`
or project `.codex/config.toml`).

```toml
[mcp_servers.smith]
url = "http://127.0.0.1:<port>/mcp"
bearer_token_env_var = "SMITH_TOKEN"   # token sourced from env var
# alternative static header:
# http_headers = { Authorization = "Bearer <token>" }
enabled = true
startup_timeout_sec = 10
tool_timeout_sec = 60
```

`[REQ-MCP-CFG-003]` Codex supports Streamable HTTP only (no legacy SSE). Smith's Streamable
HTTP endpoint satisfies this directly.

## Gemini CLI

Docs: https://github.com/google-gemini/gemini-cli/blob/main/docs/tools/mcp-server.md
(`~/.gemini/settings.json` or project `.gemini/settings.json`).

```json
{
  "mcpServers": {
    "smith": {
      "httpUrl": "http://127.0.0.1:<port>/mcp",
      "headers": { "Authorization": "Bearer <token>" },
      "timeout": 30000,
      "trust": false
    }
  }
}
```

`[REQ-MCP-CFG-004]` Gemini uses `httpUrl` for Streamable HTTP (NOT `url`, which means the
deprecated SSE transport in Gemini's config).

## Web chat (browser)

No registration file. A browser-based MCP client (Smith's own embedded chat pane, or a
third-party web client) speaks Streamable HTTP from JavaScript:

```js
// POST a JSON-RPC request, stream the SSE response
const res = await fetch("http://127.0.0.1:<port>/mcp", {
  method: "POST",
  headers: {
    "Content-Type": "application/json",
    "Accept": "application/json, text/event-stream",
    "Authorization": "Bearer <token>",   // if auth enabled
  },
  body: JSON.stringify({ jsonrpc: "2.0", id: 1, method: "initialize", params: { /* ... */ } })
});
// Read res.body as an SSE stream for server messages.
```

`[REQ-MCP-CFG-005]` For browser clients, Smith MUST:
- Apply the CORS policy of [mcp/01 REQ-MCP-014](./01-server-design.md) on `/mcp` (permissive
  `Access-Control-Allow-Origin: *` without a token; allowlist-validated `Origin` with a token).
- Respond to `OPTIONS` preflight.
- Answer Chrome Private Network Access preflights
  (`Access-Control-Request-Private-Network: true` →
  `Access-Control-Allow-Private-Network: true`) so public-origin web pages can call
  `127.0.0.1`. Reference: https://developer.chrome.com/blog/private-network-access-preflight

`[REQ-MCP-CFG-006]` Smith's own embedded chat pane (if shipped) SHOULD be served from
Smith's own HTTP origin (e.g. `http://127.0.0.1:<port>/`) so no CORS preflight is needed.

## Discovery file

Per [mcp/01-server-design.md](./01-server-design.md) §Discovery, Smith writes:

- macOS/Linux: `~/.smith/mcp.json`
- Windows: `%APPDATA%\Smith\mcp.json`

Contents:
```jsonc
{
  "version": 1,
  "pid": 12345,
  "host": "127.0.0.1",
  "port": 51234,
  "startedAt": "2026-08-09T07:00:00Z",
  "project": "/path/to/project.smith"
}
```

`[REQ-MCP-CFG-007]` Clients or wrapper scripts MAY read this file to auto-discover the
running Smith instance. Smith's CLI (`smith mcp status`) reads it to report status.

## Security checklist

Per [research/02-mcp-transport.md](../research/02-mcp-transport.md) §7:

`[REQ-MCP-CFG-008]` Smith MUST:
- Bind to `127.0.0.1` only (never `0.0.0.0`).
- Apply the token-dependent CORS policy of [mcp/01 REQ-MCP-014](./01-server-design.md):
  with a bearer token configured, validate the `Origin` header against an allowlist of
  browser origins and reject others with 403 (accept null/absent for CLI clients); with no
  token configured (loopback-only trust), permit `Access-Control-Allow-Origin: *`.
- Support an optional bearer token for defense-in-depth.
- Mark destructive tools with `annotations.destructiveHint` so hosts can gate them.

## Open questions (resolve before STABLE)

- [ ] Should Smith offer a "copy config" button per client in the UI? Yes — planned.
- [ ] Should Smith auto-register with Claude Code by writing `.mcp.json`? No — that's a
      user decision; Smith only displays the snippet. Writing the file silently is surprising.
