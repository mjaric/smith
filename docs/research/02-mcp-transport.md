---
status: DRAFT
---

# 02 — MCP HTTP/SSE Transport Research

Research reference for embedding an MCP server in Smith so that Claude Code, OpenAI Codex,
Gemini CLI, and browser-based chat clients can connect on localhost. This is a research
document, not an implementation spec; normative Smith decisions live in
[mcp/01-server-design.md](../mcp/01-server-design.md) and
[mcp/05-client-config.md](../mcp/05-client-config.md).

All requirements quoted from the MCP specification use RFC 2119 keywords as defined by the
spec. Citations are section URLs.

## 1. Specification versions

MCP revisions are date-stamped. Timeline (verified 2026-08):

| Revision | Status | Key transport/lifecycle facts |
|----------|--------|-------------------------------|
| `2024-11-05` | Superseded | First public revision. Defined **stdio** and **HTTP+SSE** (two-endpoint) transports. |
| `2025-03-26` | Superseded | Introduced **Streamable HTTP**; deprecated HTTP+SSE. |
| `2025-06-18` | Stable, widely deployed | Structured tool output (`outputSchema`/`structuredContent`), elicitation, OAuth 2.1 tightening, `MCP-Protocol-Version` header rules. |
| `2025-11-25` | Stable | OIDC discovery, icons metadata, tool-name guidance (SEP-986), experimental Tasks, JSON Schema 2020-12 default dialect, 403 for invalid `Origin`. |
| `2026-07-28` | **Latest** | Stateless core: removes `initialize` handshake and `Mcp-Session-Id`; per-request `_meta` metadata (`io.modelcontextprotocol/protocolVersion`, `clientCapabilities`); `server/discover`; `subscriptions/listen` replaces GET stream + `resources/subscribe`; MRTR replaces server-initiated requests; roots/sampling/logging **deprecated**. |

- Current spec: https://modelcontextprotocol.io/specification/2026-07-28
- Changelog 2025-11-25→2026-07-28: https://modelcontextprotocol.io/specification/2026-07-28/changelog
- Changelog 2025-06-18→2025-11-25: https://modelcontextprotocol.io/specification/2025-11-25/changelog
- Authoritative TypeScript schema: `schema/<revision>/schema.ts` in https://github.com/modelcontextprotocol/specification
- GitHub org (spec + docs + SDKs): https://github.com/modelcontextprotocol

Spec terminology: **modern** = `2026-07-28`+ (per-request metadata, no handshake);
**legacy** = `2025-11-25` and earlier (`initialize` handshake, connection/session-scoped);
**dual-era** = implements both (https://modelcontextprotocol.io/specification/2026-07-28/basic/versioning).

## 2. Transport options

Spec: https://modelcontextprotocol.io/specification/2025-06-18/basic/transports and
https://modelcontextprotocol.io/specification/2026-07-28/basic/transports. Messages are
JSON-RPC 2.0 (https://www.jsonrpc.org/), UTF-8 encoded, on every transport.

### 2.1 stdio

Client launches the server as a subprocess; newline-delimited JSON-RPC on stdin/stdout,
optional logging on stderr; nothing else may be written to stdout. Spec: clients **SHOULD**
support stdio whenever possible.
**Not applicable to Smith**: Smith is a long-running desktop application owning the model;
an MCP client cannot launch it as a child process. (A thin `smith-mcp-bridge` stdio wrapper
that proxies to the embedded HTTP server is possible later; not needed for v1.)

### 2.2 HTTP+SSE (2024-11-05) — deprecated, do not implement

Spec: https://modelcontextprotocol.io/specification/2024-11-05/basic/transports#http-with-sse.
Two endpoints: (1) an SSE endpoint the client GETs to receive server messages — first event
is `endpoint` carrying the URI to POST to; (2) a POST endpoint for client→server messages;
server messages arrive as SSE `message` events with JSON data. Deprecated since `2025-03-26`
and reclassified **Deprecated** under the feature lifecycle policy (SEP-2596): new
implementations **SHOULD NOT** adopt it; eligible for removal
(https://modelcontextprotocol.io/specification/2026-07-28/deprecated). The official Rust SDK
deliberately does not implement it.

### 2.3 Streamable HTTP (2025-03-26 → current) — the HTTP transport

Single **MCP endpoint** (e.g. `http://127.0.0.1:8390/mcp`). Legacy-era shape
(2025-03-26 … 2025-11-25), spec:
https://modelcontextprotocol.io/specification/2025-06-18/basic/transports#streamable-http:

- Client→server: every JSON-RPC message is its own HTTP POST to the MCP endpoint.
  - POST **MUST** carry `Accept: application/json, text/event-stream`.
  - Body is a single JSON-RPC *request*, *notification*, or *response*.
  - Notification/response → server **MUST** return `202 Accepted` with no body (or an HTTP
    error; body **MAY** be a JSON-RPC error with no `id`).
  - Request → server **MUST** respond either `Content-Type: application/json` (one JSON
    object) or `Content-Type: text/event-stream` (SSE stream scoped to that request).
  - On an SSE response stream the server **MAY** send related JSON-RPC requests and
    notifications (e.g. progress) before the final response, which **SHOULD** terminate the
    stream. Server **SHOULD NOT** close the stream before sending the response unless the
    session expires.
- Server→client (unprompted): client **MAY** GET the MCP endpoint (`Accept:
  text/event-stream`); server returns `text/event-stream` or `405 Method Not Allowed`. Each
  outgoing message goes to exactly one stream — **no broadcasting** across streams.
- Resumability (optional): SSE event `id` per stream; client resumes via GET with
  `Last-Event-ID`.
- Cancellation on legacy HTTP: client **SHOULD** send an explicit
  `notifications/cancelled`; disconnect **SHOULD NOT** be interpreted as cancellation.

2026-07-28 changes to Streamable HTTP
(https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http):
GET stream **removed** (use `subscriptions/listen`); sessions **removed** (no
`Mcp-Session-Id`, no DELETE); `Last-Event-ID` resumability **removed**; closing the SSE
response stream **is** the cancellation signal; required request headers `Mcp-Method` and
`Mcp-Name` (in addition to `MCP-Protocol-Version`), validated against the body — mismatch →
`400` + JSON-RPC error `-32020 HeaderMismatch`; unknown version → `400` +
`UnsupportedProtocolVersionError` (`-32022`); unknown method → `404` + `-32601`; servers
**SHOULD** send `X-Accel-Buffering: no` on SSE responses and keep-alive comment lines
(`:\r\n`) on long-lived streams; GET/DELETE from older clients → `405`.

### 2.4 Client transport support (verified 2026-08)

| Client | stdio | HTTP+SSE (legacy) | Streamable HTTP | Source |
|--------|-------|-------------------|-----------------|--------|
| Claude Code | yes (default) | yes, flagged deprecated (`--transport sse`, `"type":"sse"`) | yes, recommended (`--transport http`; JSON `type` accepts `"http"` or alias `"streamable-http"`); also nonstandard `"type":"ws"` | https://code.claude.com/docs/en/mcp |
| Codex (OpenAI) | yes (`command`/`args`/`env`) | no documented support | yes — `mcp_servers.<id>.url` is documented as "Endpoint for an MCP streamable HTTP server" | https://learn.chatgpt.com/docs/config-file/config-reference |
| Gemini CLI | yes (`command`) | yes (`url` = "SSE endpoint URL") | yes (`httpUrl` = "HTTP streaming endpoint URL") | https://github.com/google-gemini/gemini-cli/blob/main/docs/tools/mcp-server.md |
| Web chat (browser) | n/a | possible via EventSource | yes — any browser MCP client library speaks it (fetch POST + streaming SSE body) | MCP TS SDK; CORS/PNA constraints §7 |

[INFERENCE] The exact protocol revision each CLI client negotiates is not documented by
Anthropic/OpenAI/Google; Claude Code and Gemini CLI are built on the official TypeScript
SDK, whose support spans 2024-11-05 … 2025-11-25 and is adding 2026-07-28. Therefore the
installed base is guaranteed to speak **legacy-era Streamable HTTP** (handshake + optional
sessions), which is what Smith must target first.

### 2.5 Recommendation for Smith

Implement **Streamable HTTP, legacy era** (protocol versions `2025-06-18` and
`2025-11-25`), on a single MCP endpoint `http://127.0.0.1:<port>/mcp`:

1. POST handling per §2.3 (JSON or per-request SSE response).
2. GET SSE stream enabled — this is the server→client channel Smith needs for
   `notifications/tools/list_changed`, `notifications/resources/list_changed`, and
   `notifications/resources/updated` pushed while an agent session is idle.
3. Optional `Mcp-Session-Id` sessions (§3); stateless per-request operation also acceptable.
4. Do **not** implement the deprecated two-endpoint HTTP+SSE transport (deprecated since
   2025-03-26; clients that still need it POST-probe then GET-probe — a 405 on GET /sse is
   a valid negative).
5. Do not implement stdio in v1.
6. Design the dispatch layer era-aware so `2026-07-28` (per-request `_meta`,
   `subscriptions/listen`, `server/discover`) can be added without rework; the versioning
   compatibility matrix says a legacy server simply rejects modern requests with a
   JSON-RPC error and modern clients fall back or report it
   (https://modelcontextprotocol.io/specification/2026-07-28/basic/versioning#backward-compatibility-with-initialization-based-versions).

Rationale: localhost + SSE streaming requirement maps exactly to Streamable HTTP; it is the
only non-deprecated HTTP transport; all three target CLI clients support it; the official
Rust SDK ships it as a mountable Tower service (§8).

## 3. Server lifecycle & endpoints (legacy-era Streamable HTTP)

Spec: https://modelcontextprotocol.io/specification/2025-06-18/basic/lifecycle +
transports page §"Session Management" / §"Protocol Version Header".

Phases: **Initialization** (must be first interaction) → **Operation** → **Shutdown**
(close HTTP connections; no shutdown messages).

### 3.1 Initialization handshake

Client POSTs `initialize`:

```json
{
  "jsonrpc": "2.0", "id": 1, "method": "initialize",
  "params": {
    "protocolVersion": "2025-06-18",
    "capabilities": { "roots": { "listChanged": true }, "sampling": {}, "elicitation": {} },
    "clientInfo": { "name": "ExampleClient", "version": "1.0.0" }
  }
}
```

Server responds `InitializeResult` (see §5 for Smith's exact shape), then client POSTs
`{"jsonrpc":"2.0","method":"notifications/initialized"}` → `202 Accepted`. Version
negotiation: if the server supports the requested version it **MUST** echo it, else it
**MUST** respond with the latest version it supports; a client that cannot speak that
version **SHOULD** disconnect. Before `initialized`, only pings (and server logging) are
allowed.

### 3.2 Session management (`Mcp-Session-Id`)

- Server **MAY** assign a session ID at initialization by returning an `Mcp-Session-Id`
  header on the `InitializeResult` HTTP response.
- ID **SHOULD** be globally unique and cryptographically secure (UUID, JWT, hash); **MUST**
  be visible ASCII 0x21–0x7E.
- If issued, client **MUST** send `Mcp-Session-Id` on all subsequent requests; server
  **SHOULD** answer requests lacking it (other than initialization) with `400 Bad Request`.
- Server may terminate a session anytime; then **MUST** answer that ID with `404 Not
  Found`; client **MUST** start a fresh session (new `initialize`, no session header).
- Client **SHOULD** end sessions via HTTP DELETE with the header; server **MAY** answer
  `405` if it disallows explicit termination.

### 3.3 `MCP-Protocol-Version` header

From 2025-06-18: client **MUST** send `MCP-Protocol-Version: <version>` on all HTTP
requests after initialization (value = negotiated version). Missing header → server
**SHOULD** assume `2025-03-26` for backward compat; invalid/unsupported → `400 Bad
Request`.

### 3.4 Request/response shape on the wire (example)

```http
POST /mcp HTTP/1.1
Host: 127.0.0.1:8390
Content-Type: application/json
Accept: application/json, text/event-stream
Mcp-Session-Id: 1868a90c-...
MCP-Protocol-Version: 2025-11-25

{"jsonrpc":"2.0","id":2,"method":"tools/call",
 "params":{"name":"create_class","arguments":{"name":"Order"}}}
```

Response is either one JSON object (`application/json`) or an SSE stream
(`text/event-stream`) whose events carry JSON-RPC messages as `data:` payloads, ending with
the response for id 2.

### 3.5 What changes in 2026-07-28 (future-proofing)

No handshake: every request carries `_meta` fields `io.modelcontextprotocol/protocolVersion`,
`io.modelcontextprotocol/clientCapabilities`, and SHOULD carry `clientInfo`; servers MUST
implement `server/discover` (advertises versions/capabilities/identity); results carry
required `resultType` (`complete` | `input_required`); server→client interactions arrive as
MRTR `InputRequiredResult.inputRequests` (client retries the request with `inputResponses`);
long-lived change-notification streams come from POSTing `subscriptions/listen` (opt-in
categories `toolsListChanged`, `promptsListChanged`, `resourcesListChanged`,
`resourceSubscriptions`); list/read results carry required `ttlMs`/`cacheScope`
(https://modelcontextprotocol.io/specification/2026-07-28/changelog).

## 4. Protocol primitives Smith must implement

### 4.1 Tools (model-controlled) — PRIMARY surface

Spec: https://modelcontextprotocol.io/specification/2025-06-18/server/tools. This is how an
agent manipulates Smith's model (create class, add method, run RTM).

- Capability: `{"tools": {"listChanged": true}}`.
- `tools/list` (paginated via `cursor`/`nextCursor`) → `{tools: [...], nextCursor?}`; each
  tool: `name` (unique), `title?`, `description?`, `inputSchema` (JSON Schema object),
  `outputSchema?` (2025-06-18+, enables `structuredContent` in results), `annotations?`
  (`readOnlyHint`, `destructiveHint`, `idempotentHint`, `openWorldHint` — clients MUST
  treat annotations as untrusted).
- `tools/call` `{name, arguments, _meta.progressToken?}` → `{content: [text|image|audio|
  resource_link|resource], structuredContent?, isError}`. Tool execution errors (incl. input
  validation, per 2025-11-25 SEP-1303) go in the result with `isError: true` so the model
  can self-correct; protocol errors use JSON-RPC error objects.
- `notifications/tools/list_changed` when declared.
- Tool-name format guidance added in 2025-11-25 (SEP-986); 2026-07-28 adds `x-mcp-header`
  parameter→header mirroring (clients must support; Smith need not use it).

### 4.2 Resources (application-driven)

Spec: https://modelcontextprotocol.io/specification/2025-06-18/server/resources. Expose
model elements, diagrams (as views), and analysis results as readable resources.

- Capability: `{"resources": {"subscribe": true?, "listChanged": true?}}` (both optional).
- `resources/list` (paginated) → `{resources: [{uri, name, title?, description?, mimeType?, size?}]}`. Each resource URI is unique; Smith defines its own scheme (e.g.
  `smith://elements/{id}`, `smith://diagrams/{id}`, `smith://analysis/rtm`).
- `resources/read {uri}` → `{contents: [{uri, mimeType, text | blob(base64)}]}`.
- `resources/templates/list` → RFC 6570 URI templates (e.g.
  `smith://elements/{elementId}`) — the right tool for Smith's open-ended element space;
  arguments may be autocompleted via `completion/complete`.
- `resources/subscribe {uri}` / `resources/unsubscribe {uri}` (legacy era) → server pushes
  `notifications/resources/updated {uri}` on an open stream; client re-reads. Replaced by
  `subscriptions/listen` in 2026-07-28.
- `notifications/resources/list_changed`.
- Resource-not-found error code: `-32002` in legacy revisions → `-32602` (Invalid Params)
  in 2026-07-28.

### 4.3 Prompts (user-controlled)

Spec: https://modelcontextprotocol.io/specification/2025-06-18/server/prompts. Predefined
prompts for common modeling tasks (e.g. "derive use cases from requirement", "build RTM
gap report"); clients typically surface them as slash commands.

- Capability: `{"prompts": {"listChanged": true}}`.
- `prompts/list` (paginated) → `{prompts: [{name, title?, description?, arguments: [{name, description?, required?}]}]}`.
- `prompts/get {name, arguments}` → `{description?, messages: [{role: "user"|"assistant", content: text|image|audio|resource}]}`.
- `notifications/prompts/list_changed`.
- Errors: invalid name / missing required argument → `-32602`.

### 4.4 Roots (client→server, optional)

Spec: https://modelcontextprotocol.io/specification/2025-06-18/client/roots. Client exposes
filesystem roots (file:// URIs); server requests `roots/list`, receives
`notifications/roots/list_changed`. Claude Code answers with the launch directory plus
`--add-dir` working directories. **Deprecated in 2026-07-28 (SEP-2577)** — pass paths via
tool arguments/resource URIs instead. Smith MAY consume roots to resolve relative paths in
agent requests; low priority for v1.

### 4.5 Sampling (client→server, optional)

Spec: https://modelcontextprotocol.io/specification/2025-06-18/client/sampling. Server asks
the client's LLM for a completion via `sampling/createMessage` (messages, `modelPreferences`
with hints + cost/speed/intelligence priorities, `systemPrompt`, `maxTokens`) → `{role,
content, model, stopReason}`; human-in-the-loop review is REQUIRED of clients. **Deprecated
in 2026-07-28 (SEP-2577).** Smith does not need sampling in v1 — the agent already owns its
LLM. (2025-11-25 added `tools`/`toolChoice` to sampling; irrelevant here.)

### 4.6 Elicitation (client→server, optional)

Added 2025-06-18 (https://modelcontextprotocol.io/specification/2025-06-18/client/elicitation).
Server requests structured user input via `elicitation/create` with a JSON schema; client
capability `{"elicitation": {}}`. Useful if Smith wants agents to confirm destructive
operations through the host UI. Optional.

### 4.7 Notifications, progress, cancellation

- Progress (https://modelcontextprotocol.io/specification/2025-06-18/basic/utilities/progress):
  client passes `params._meta.progressToken` (string|integer, unique per active request);
  server MAY emit `notifications/progress {progressToken, progress, total?, message?}`;
  `progress` MUST increase; sent on the request's SSE response stream; MUST stop after
  completion. Smith SHOULD emit these for long analyses (RTM generation, batch imports).
- Cancellation (https://modelcontextprotocol.io/specification/2025-06-18/basic/utilities/cancellation):
  `notifications/cancelled {requestId, reason?}`; receiver SHOULD stop work and send no
  response; races MUST be handled gracefully. Legacy HTTP: disconnect ≠ cancel; 2026-07-28
  HTTP: closing the response stream **is** cancellation.
- Resource updates: `notifications/resources/updated` (see §4.2).
- Keep-alive: legacy era has `ping`; removed in 2026-07-28.

### 4.8 JSON-RPC error codes

Standard: -32700 parse, -32600 invalid request, -32601 method not found, -32602 invalid
params, -32603 internal. MCP-reserved server range: `-32000…-32019` implementation-defined,
`-32020…-32099` spec-reserved (2026-07-28): `-32020 HeaderMismatch`, `-32021
MissingRequiredClientCapability`, `-32022 UnsupportedProtocolVersion`.

## 5. Capability negotiation — Smith's initialize response

Spec: https://modelcontextprotocol.io/specification/2025-06-18/basic/lifecycle#initialization.

Server capability keys: `prompts`, `resources`, `tools`, `logging`, `completions`,
`experimental` (each with `listChanged`/`subscribe` sub-capabilities as applicable). Client
keys: `roots`, `sampling`, `elicitation`, `experimental`. Both sides MUST only use
negotiated capabilities during operation.

Example `InitializeResult` Smith should return (legacy era):

```json
{
  "jsonrpc": "2.0", "id": 1,
  "result": {
    "protocolVersion": "2025-11-25",
    "capabilities": {
      "logging": {},
      "completions": {},
      "prompts":   { "listChanged": true },
      "resources": { "subscribe": true, "listChanged": true },
      "tools":     { "listChanged": true }
    },
    "serverInfo": { "name": "smith", "title": "Smith UML Modeler", "version": "1.0.0" },
    "instructions": "Smith exposes the open UML model. Element IDs are stable UUIDs; use smith:// resources to read, tools to mutate, and analytics tools for RTM/orphan/coverage checks."
  }
}
```

`instructions` (free text) is injected into the model's context by hosts — Smith should use
it to teach tool conventions and the canonical
`Requirement → UseCase → Activity → Sequence → Class` chain. 2025-11-25 adds an optional
`description` field to `Implementation` (serverInfo/clientInfo).

## 6. Client configuration formats

### 6.1 Claude Code

Docs: https://code.claude.com/docs/en/mcp.

```bash
# Streamable HTTP (recommended transport)
claude mcp add --transport http smith http://127.0.0.1:8390/mcp
# with auth header
claude mcp add --transport http smith http://127.0.0.1:8390/mcp \
  --header "Authorization: Bearer ${SMITH_TOKEN}"
# legacy SSE endpoint (deprecated transport)
claude mcp add --transport sse smith http://127.0.0.1:8390/sse
```

Project-scoped `.mcp.json` (checked into project root; requires interactive approval):

```json
{
  "mcpServers": {
    "smith": {
      "type": "http",
      "url": "http://127.0.0.1:8390/mcp",
      "headers": { "Authorization": "Bearer ${SMITH_TOKEN}" },
      "timeout": 600000
    }
  }
}
```

Facts that matter for Smith:
- `type` accepts `"http"` or alias `"streamable-http"` (spec name); `"sse"` for the legacy
  transport; an entry with `url` but **no** `type` is a config error (defaults to stdio
  interpretation) — Smith's docs must always include `type`.
- Scopes: `local` (default, per-project in `~/.claude.json`), `project` (`.mcp.json`),
  `user` (`--scope user`, `~/.claude.json`). `${VAR}` / `${VAR:-default}` expansion in
  `url`/`headers`/`env`/`command`/`args`.
- HTTP/SSE servers auto-reconnect with exponential backoff (5 attempts); initial connection
  retried 3×. Supports `list_changed` notifications (dynamic tool refresh). OAuth via `/mcp`
  for remote servers; localhost servers typically use static headers.

### 6.2 Codex (OpenAI)

Docs: https://learn.chatgpt.com/docs/config-file/config-reference (`~/.codex/config.toml`,
project overrides `.codex/config.toml`). Codex supports **streamable HTTP** (no legacy SSE
endpoint mode documented):

```toml
[mcp_servers.smith]
url = "http://127.0.0.1:8390/mcp"
# optional auth
bearer_token_env_var = "SMITH_TOKEN"     # token sourced from env var
# http_headers = { X-Api-Key = "..." }   # static headers
# env_http_headers = { Authorization = "SMITH_AUTH" }
# auth = "oauth"                          # fallback after configured tokens
enabled = true
# required = true                         # fail startup if Smith unreachable
startup_timeout_sec = 10
tool_timeout_sec = 60
# enabled_tools / disabled_tools          # allow/deny lists
```

Stdio variant uses `command`/`args`/`env`/`cwd`. Codex also supports per-tool approval
modes and MCP elicitation gating (`approval_policy.granular.mcp_elicitations`).

### 6.3 Gemini CLI

Docs: https://github.com/google-gemini/gemini-cli/blob/main/docs/tools/mcp-server.md
(`settings.json`: user `~/.gemini/settings.json`, project `.gemini/settings.json`).
Supports stdio, SSE, and Streamable HTTP:

```json
{
  "mcpServers": {
    "smith": {
      "httpUrl": "http://127.0.0.1:8390/mcp",
      "headers": { "Authorization": "Bearer $SMITH_TOKEN" },
      "timeout": 30000,
      "trust": false
    }
  }
}
```

Key semantics: `url` = **SSE** endpoint URL (legacy transport, e.g.
`http://localhost:8080/sse`); `httpUrl` = **Streamable HTTP** endpoint. Also `command`/
`args`/`env`/`cwd` (stdio), `includeTools`/`excludeTools` allow/deny lists, `trust: true`
bypasses per-call confirmation. Resources are discoverable and referenceable in chat via
`@server://resource/path`, which triggers `resources/read`.

### 6.4 Web chat (browser client)

No standard registration file — a browser-based MCP client (e.g. Smith's own embedded chat
pane, or the MCP Inspector web client) speaks Streamable HTTP from JavaScript:
`fetch` POST to `/mcp` with `Accept: application/json, text/event-stream`, streaming the SSE
response body (or `EventSource` for a legacy GET stream).

Browser-specific constraints:
- **CORS**: cross-origin web pages need `Access-Control-Allow-Origin` (+ headers exposure)
  from Smith; serving the chat page from Smith's own origin (recommended) avoids CORS
  entirely.
- **Chrome Private Network Access / Local Network Access**: public websites calling
  `127.0.0.1` trigger an OPTIONS preflight carrying
  `Access-Control-Request-Private-Network: true`; the server must answer
  `Access-Control-Allow-Private-Network: true` (Chrome ≥94 warning/≥127 enforcing; Chrome
  142+ evolves this into LNA with a user permission prompt)
  (https://developer.chrome.com/blog/private-network-access-preflight). Smith must answer
  these preflights for any remote-origin web client.

## 7. Auth & security on localhost

Spec security warnings for Streamable HTTP
(https://modelcontextprotocol.io/specification/2025-06-18/basic/transports#security-warning,
2026-07-28 §Security & Endpoint):

1. Servers **MUST** validate the `Origin` header on all incoming connections to prevent
   DNS-rebinding attacks; 2025-11-25 clarifies: present-and-invalid `Origin` → **MUST**
   respond `403 Forbidden`.
2. When running locally, servers **SHOULD** bind only to `127.0.0.1`, never `0.0.0.0`.
3. Servers **SHOULD** implement proper authentication for all connections.

Threat model: a malicious website open in the user's browser can resolve an attacker domain
to 127.0.0.1 (DNS rebinding) and drive Smith's endpoint — `Origin` validation plus
loopback-only binding is the spec-mandated defense; additionally check the `Host` header.

Recommendations for Smith:
- **Bind 127.0.0.1 only** (config may allow a user-chosen loopback port; never all
  interfaces).
- **Origin allowlist**: accept null/absent origins (non-browser CLI clients send none) and
  configured origins (Smith's own served pages); reject everything else with 403.
- **Optional bearer token**: generate a per-installation random token, surface it in Smith's
  UI for pasting into client configs (`headers.Authorization` / `bearer_token_env_var`).
  Defense-in-depth against other local processes; not a substitute for loopback binding.
  http:// on loopback is acceptable to all three CLI clients (no TLS requirement for
  localhost in their docs).
- **Human-in-the-loop**: spec says hosts SHOULD gate tool calls; Smith's mutating tools
  SHOULD additionally surface confirmations in its own UI (tools spec trust & safety
  warning). Mark destructive tools with `annotations.destructiveHint`.
- Tool/prompt inputs MUST be validated (injection), per prompts spec security section.
- Full guidance: https://modelcontextprotocol.io/docs/2026-07-28/tutorials/security/security_best_practices
  (confused deputy, token passthrough — mostly OAuth/remote concerns; Smith's localhost
  posture sidesteps most of them).

## 8. Rust SDK status

**Official Rust SDK exists and is mature**: `modelcontextprotocol/rust-sdk` → crate `rmcp`
(https://crates.io/crates/rmcp, https://github.com/modelcontextprotocol/rust-sdk, ~3.8k
stars, tokio/serde/schemars based).

- Implements the **2026-07-28** spec while remaining compatible with **2025-11-25** and
  earlier; ships a conformance test suite with published assessment results
  (`conformance/results/2026-02-25-*`). The SDK tiering system (SEP-1730, tiers published
  2026-02-23; https://modelcontextprotocol.io/community/sdk-tiers) sets conformance
  expectations for official SDKs.
- Server API: implement `ServerHandler`; `#[tool]`/`#[tool_router]` macros generate tool
  routers with JSON Schema (2020-12) generation via schemars; prompt/resource routers exist.
- **Streamable HTTP server is a Tower service** mounted on any axum/hyper router:

```rust
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, StreamableHttpServerConfig,
    session::local::LocalSessionManager,
};

let config = StreamableHttpServerConfig::default()
    .with_legacy_session_mode(false) // or true for sessionful legacy clients
    .with_json_response(true);       // plain JSON replies for simple tools

let service = StreamableHttpService::new(
    || Ok(SmithMcpServer::new(model.clone())), // handler factory
    LocalSessionManager::default().into(),
    config,
);
let router = axum::Router::new().nest_service("/mcp", service);
```

- Feature flags: `server`, `transport-io` (stdio), `transport-streamable-http-server`,
  `transport-streamable-http-client-reqwest`, `transport-child-process`, `transport-worker`.
- **Deliberate non-goal**: the legacy two-endpoint HTTP+SSE transport (2024-11-05) is not
  implemented and will not be added — consistent with §2.5 recommendation.

Community alternatives exist (e.g. older `rust-mcp-sdk`), but the official `rmcp` is the
only one tracking current revisions with conformance testing.

**Recommendation**: use `rmcp` (features `server` + `transport-streamable-http-server`)
mounted on Smith's axum server rather than hand-rolling JSON-RPC-over-HTTP/SSE. The raw
protocol is simple (JSON-RPC 2.0 + SSE), but rmcp supplies schema-faithful types, session
management, SSE framing, era handling, and conformance-tested behavior. If Smith needs a
legacy-era behavior rmcp doesn't expose, the escape hatch is implementing the §3 wire
protocol directly on axum — the shapes above are the complete contract.

## Sources

- MCP specification (latest): https://modelcontextprotocol.io/specification/2026-07-28
- 2026-07-28 changelog: https://modelcontextprotocol.io/specification/2026-07-28/changelog
- 2025-11-25 changelog: https://modelcontextprotocol.io/specification/2025-11-25/changelog
- 2026-07-28 transports overview: https://modelcontextprotocol.io/specification/2026-07-28/basic/transports
- 2026-07-28 Streamable HTTP binding: https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http
- 2026-07-28 versioning & compatibility: https://modelcontextprotocol.io/specification/2026-07-28/basic/versioning
- Deprecated features registry: https://modelcontextprotocol.io/specification/2026-07-28/deprecated
- 2025-06-18 transports (Streamable HTTP + security warning): https://modelcontextprotocol.io/specification/2025-06-18/basic/transports
- 2025-06-18 lifecycle (initialize/capabilities): https://modelcontextprotocol.io/specification/2025-06-18/basic/lifecycle
- 2024-11-05 HTTP+SSE transport: https://modelcontextprotocol.io/specification/2024-11-05/basic/transports
- Tools: https://modelcontextprotocol.io/specification/2025-06-18/server/tools
- Resources: https://modelcontextprotocol.io/specification/2025-06-18/server/resources
- Prompts: https://modelcontextprotocol.io/specification/2025-06-18/server/prompts
- Roots: https://modelcontextprotocol.io/specification/2025-06-18/client/roots
- Sampling: https://modelcontextprotocol.io/specification/2025-06-18/client/sampling
- Progress: https://modelcontextprotocol.io/specification/2025-06-18/basic/utilities/progress
- Cancellation: https://modelcontextprotocol.io/specification/2025-06-18/basic/utilities/cancellation
- Specification repo & TypeScript schema: https://github.com/modelcontextprotocol/specification
- MCP org & docs repo: https://github.com/modelcontextprotocol/modelcontextprotocol
- Official Rust SDK (rmcp): https://github.com/modelcontextprotocol/rust-sdk · https://crates.io/crates/rmcp
- SDK tiering system: https://modelcontextprotocol.io/community/sdk-tiers
- Security best practices: https://modelcontextprotocol.io/docs/2026-07-28/tutorials/security/security_best_practices
- Claude Code MCP reference: https://code.claude.com/docs/en/mcp
- Codex configuration reference: https://learn.chatgpt.com/docs/config-file/config-reference
- Gemini CLI MCP servers: https://github.com/google-gemini/gemini-cli/blob/main/docs/tools/mcp-server.md
- JSON-RPC 2.0: https://www.jsonrpc.org/
- Server-Sent Events: https://html.spec.whatwg.org/multipage/server-sent-events.html
- Chrome Private Network Access preflights: https://developer.chrome.com/blog/private-network-access-preflight
