---
status: DRAFT
---

# MCP 01 — Server Design

Smith embeds an **MCP server** in-process. Any MCP-capable client (Claude Code, OpenAI Codex,
Google Gemini CLI, a browser chat) connects to the running Smith instance over HTTP/SSE on
`127.0.0.1` and reads or mutates the model through typed tools, reads model resources, and
invokes predefined prompts.

The MCP server and the UI call the **same** underlying model API (design principle P3).
There is no agent-only or UI-only capability. Every model mutation from an agent enters the
same undo stack as a UI action.

Background: [research/02-mcp-transport.md](../research/02-mcp-transport.md).

## Transport

`[REQ-MCP-001]` Smith MUST implement the **Streamable HTTP** transport (MCP spec
2025-03-26 and later) over HTTP/1.1 on `127.0.0.1`. The legacy SSE transport MAY also be
served for older clients that do not support Streamable HTTP; both share the same endpoint
set.

`[REQ-MCP-002]` The server MUST bind to `127.0.0.1` (loopback) only. It MUST NOT bind to
`0.0.0.0` or any routable interface. Configuration to change the bind address is NOT
exposed in v1.

`[REQ-MCP-003]` The port is OS-assigned ephemeral by default, written to the discovery file
on startup (see §Discovery). A configurable fixed port (default `7243`) MAY be set in
settings; when set, Smith binds it directly.

`[REQ-MCP-004]` The server MUST speak JSON-RPC 2.0 over the HTTP transport, per the MCP
spec. Every request and response is a JSON-RPC message.

## Endpoints

| Path | Method | Purpose |
|------|--------|---------|
| `/mcp` | POST | Client → server request (JSON-RPC). Single request per POST. |
| `/mcp` | GET | Server → client stream (SSE), opened by client to receive server-initiated messages and responses for long operations. |
| `/mcp` | DELETE | Session termination (Streamable HTTP). |

`[REQ-MCP-005]` The server MUST support session resumption via the `Mcp-Session-Id`
header returned on `initialize` and echoed by the client on subsequent requests.

`[REQ-MCP-006]` The server MUST emit `notifications/resources/updated` on the SSE stream
whenever a model resource changes, so connected agents can re-fetch without polling.

## Capability negotiation

On `initialize`, Smith advertises:

```jsonc
{
  "protocolVersion": "2025-06-18",
  "capabilities": {
    "tools": { "listChanged": true },
    "resources": { "listChanged": true, "subscribe": true },
    "prompts": { "listChanged": true },
    "logging": {}
  },
  "serverInfo": { "name": "smith", "version": "<semver>" }
}
```

`[REQ-MCP-007]` Smith MUST implement `tools/list`, `tools/call`, `resources/list`,
`resources/read`, `resources/subscribe`, `resources/unsubscribe`, `prompts/list`,
`prompts/get`, and the `initialize` / `notifications/*` lifecycle methods.

`[REQ-MCP-008]` Smith SHOULD implement `logging/setLevel` and emit structured log
notifications for significant model events (element created, relation added, analysis run).

## Session lifecycle

1. Client opens HTTP connection, sends `initialize`.
2. Server responds with capabilities + `Mcp-Session-Id`.
3. Client sends `notifications/initialized`.
4. Client may now call `tools/*`, `resources/*`, `prompts/*`.
5. Client may `resources/subscribe` to specific resource URIs for change notifications.
6. Session ends on HTTP connection close or explicit `DELETE` on `/mcp`.

`[REQ-MCP-009]` Multiple concurrent sessions MUST be supported (Claude Code and a web chat
connected simultaneously). The model is shared; sessions differ only in their notification
subscriptions and capability preferences.

`[REQ-MCP-010]` Every model mutation from a tool call MUST be attributable to the session
that performed it (recorded in the command/undo history with an `actor` field: `ui` or
`session:<id>`).

## Discovery

For clients that need to find the running Smith server, Smith writes a discovery file:

- **macOS/Linux:** `~/.smith/mcp.json`
- **Windows:** `%APPDATA%\Smith\mcp.json`

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

`[REQ-MCP-011]` Smith MUST write this file on server start and MUST remove it on clean
exit. A stale file (PID not running) is overwritten on next start.

`[REQ-MCP-012]` Smith MUST regenerate the file if the project changes (open a different
project) so connected clients can re-bind.

## Authentication & security

`[REQ-MCP-013]` The server MUST accept an optional bearer token (`Authorization: Bearer
<token>`) configured in Smith's settings. If configured, every request MUST present the
token or be rejected with HTTP 401. If not configured, no auth is required (loopback-only
binding is the security boundary).

`[REQ-MCP-014]` For browser-based chat clients, the server MUST send CORS headers and
respond to `OPTIONS` preflight on the `/mcp` endpoint. The policy depends on whether a
bearer token is configured (REQ-MCP-013):
- **No token configured** (loopback-only trust): permit `Access-Control-Allow-Origin: *`.
- **Token configured**: validate the request `Origin` against an allowlist of permitted
  browser origins and reflect the matched origin as `Access-Control-Allow-Origin`; reject
  origins not on the allowlist with HTTP 403.

`[REQ-MCP-015]` The server MUST NOT expose filesystem-escape or shell-execution tools. The
MCP surface is the model API only.

## Error model

Tool and resource errors use MCP's standard error structure. Smith defines these error
codes (namespace `smith`):

| Code | Meaning |
|------|---------|
| `-32000` | Element not found (bad ID). |
| `-32001` | Metamodel constraint violation: wrong element kinds for a relation stereotype, applying a stereotype to a wrong metaclass, or adding a view reference to an element kind disallowed on that diagram kind. |
| `-32002` | Uniqueness violation: name collision in namespace, duplicate project-wide `requirementId`, or duplicate `testCaseId`. |
| `-32003` | Operation would create a model defect (e.g. cycle); the operation is rejected unless `acceptDefect: true` is passed. |
| `-32004` | Project read-only / locked. |
| `-32005` | Undo stack exhausted. |

`[REQ-MCP-016]` Every tool error MUST include a human-readable `message` and a `data`
object with at least `{ "code": "<stable-code>" }`.

## Undo & batching

`[REQ-MCP-017]` Every model-mutating tool call pushes one command onto the shared undo
stack. A tool that performs multiple related mutations MAY wrap them in a single
`batch` command so they undo as one unit.

`[REQ-MCP-018]` The `batch.undo` and `batch.redo` tools MUST be exposed so an agent can
reverse its own actions; the UI's undo button and the agent's `batch.undo` tool operate on
the same stack.

## Concurrency

`[REQ-MCP-019]` The model API MUST be single-writer (all mutations serialize through one
async task / mutex). Reads are concurrent. This prevents interleaved mutations from
different sessions from corrupting the model.

`[REQ-MCP-020]` A long-running analysis tool (e.g. full RTM on a huge model) MUST run off
the writer thread and MUST be cancellable via a `cancel` tool taking the operation's
`progressToken`.

## Transport selection rationale

Streamable HTTP (not stdio) is chosen because:

1. Multiple clients connect simultaneously (Claude Code + a web chat). stdio allows exactly
   one client.
2. Browser-based chat clients cannot spawn a stdio subprocess; they need HTTP.
3. The server is long-lived (Smith's lifetime), not spawned per agent session. stdio servers
   are conventionally launched by the client; Smith is launched by the user.

## Open questions (resolve before STABLE)

- [ ] Should Smith support the older pure-SSE transport for back-compat with clients that
      only speak the 2024-11-05 spec? Decision: yes, as a thin adapter, marked deprecated.
- [ ] Exact port allocation strategy: fixed default vs. ephemeral + discovery file. Current
      spec: ephemeral + discovery; fixed default as fallback.
