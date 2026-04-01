# Hyperlight VM

Run JavaScript inside [Hyperlight](https://github.com/hyperlight-dev/hyperlight) micro-VMs using KVM.

## Prerequisites

- **Rust 1.89+** — install via [rustup](https://rustup.rs)
- **Linux with KVM** — a bare-metal Linux host or a VM with nested virtualization enabled

### KVM Setup

> **Note:** If you're using the included [dev container](#dev-container), KVM access is configured automatically — skip this section.

Hyperlight requires access to `/dev/kvm`. Add your user to the `kvm` group:

```bash
sudo usermod -aG kvm $USER
```

Then **log out and back in** for the group change to take effect. Verify with:

```bash
groups | grep kvm
```

> **Tip:** To apply the group in your current shell without logging out, use:
> ```bash
> newgrp kvm
> ```

## Project Structure

| Directory | Description |
|-----------|-------------|
| `js-host/` | Runs JavaScript inside a Hyperlight micro-VM via [hyperlight-js](https://github.com/hyperlight-dev/hyperlight-js), with a host plugin system |
| `js-host/src/plugins/` | Host-side plugins (math, time, kv) callable from guest JS via ES module imports |

## Running

Start the REST API server:

```bash
cd js-host && cargo run --release
```

The server listens on `http://127.0.0.1:8888` by default. Override with the `BIND_ADDR` environment variable:

```bash
BIND_ADDR=0.0.0.0:3000 cargo run --release
```

### API

#### `POST /execute`

Execute JavaScript inside a Hyperlight micro-VM. Each request gets a fresh sandboxed VM.

**Request body** (JSON):

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `code` | string | yes | JavaScript source code. Must export a `handler` function. |
| `event` | object | no | JSON payload passed to the handler (defaults to `{}`). |

**Example:**

```bash
curl -s -X POST http://127.0.0.1:8888/execute \
  -H 'Content-Type: application/json' \
  -d '{
    "code": "function handler(e) { return { sum: e.a + e.b }; }\nexport { handler };",
    "event": {"a": 3, "b": 4}
  }'
```

**Success response** (`200`):

```json
{"result": {"sum": 7}}
```

**Error response** (`400` for JS errors, `500` for internal errors):

```json
{"error": "description of what went wrong"}
```

**Using host plugins from JS:**

```bash
curl -s -X POST http://127.0.0.1:8888/execute \
  -H 'Content-Type: application/json' \
  -d '{
    "code": "import * as math from \"math\";\nfunction handler(e) { return { result: math.sqrt(e.x) }; }\nexport { handler };",
    "event": {"x": 144}
  }'
```

```json
{"result": {"result": 12}}
```

#### Host Plugins

Plugins are Rust functions registered as host modules on the sandbox **before** the JS runtime loads. Guest JavaScript imports them as ES modules (e.g. `import * as math from "math"`). This follows the same pattern used in [hyperlight](https://github.com/hyperlight-dev/hyperlight) and [hyperagent](https://github.com/hyperlight-dev/hyperagent).

| Plugin | Module | Functions |
|--------|--------|-----------|
| **math** | `"math"` | `sqrt`, `pow`, `abs`, `floor`, `ceil`, `round`, `log`, `min`, `max` |
| **time** | `"time"` | `now_ms` (epoch millis), `now_secs` (epoch seconds) |
| **kv** | `"kv"` | `set`, `get`, `delete`, `keys` (in-memory key-value store) |
| **indices** | `"indices"` | `get` (fetch index data from the indices service) |

To add a new plugin, create a struct implementing the `Plugin` trait in `js-host/src/plugins/` and register it in `all_plugins()`. No changes to `main.rs` needed.

**Querying the indices plugin:**

```bash
curl -s -X POST http://127.0.0.1:8888/execute \
  -H 'Content-Type: application/json' \
  -d '{
    "code": "import * as indices from \"indices\";\nasync function handler(e) { const result = await indices.get(e.symbol); return { data: result }; }\nexport { handler };",
    "event": {"symbol": "SPX"}
  }'
```

```
SandboxBuilder::new().build()  →  ProtoJSSandbox
        ↓  plugin.register(&mut proto)      ← plugins registered here
proto.load_runtime()           →  JSSandbox  (registrations frozen)
        ↓
sandbox.add_handler() / get_loaded_sandbox()
        ↓
loaded.handle_event()          →  JSON result
```

## Dev Container

This repo includes a [dev container](.devcontainer/devcontainer.json) configuration for VS Code. It uses the official Hyperlight devcontainer image with KVM passthrough, so you get a ready-to-go environment with Rust and all build tools pre-installed.

**Requirements:** Your Docker host must have `/dev/kvm` available (bare-metal Linux or a VM with nested virtualization).

To use it:
1. Open this repo in VS Code
2. Install the [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension
3. Press `F1` → **Dev Containers: Reopen in Container**

KVM access is configured automatically — no manual group setup needed inside the container.

## Troubleshooting

| Error | Fix |
|-------|-----|
| `No Hypervisor was found for Sandbox` | Run `sudo usermod -aG kvm $USER` then log out/in, or use `newgrp kvm` |
| `/dev/kvm` not found | Enable KVM in your BIOS/hypervisor (Intel VT-x / AMD-V) and load the module: `sudo modprobe kvm_intel` or `sudo modprobe kvm_amd` |
| Permission denied on `/dev/kvm` | Check permissions: `ls -la /dev/kvm` — ensure your user is in the `kvm` group |
