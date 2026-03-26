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

Run JavaScript code inside a Hyperlight micro-VM with QuickJS:

```bash
cd js-host && cargo run --release
```

The built-in handler demonstrates three host plugins — **math**, **time**, and **kv** — all callable from guest JavaScript via ES module imports:

```
  ⚙ Registering plugin: math
  ⚙ Registering plugin: time
  ⚙ Registering plugin: kv
{
  "math": {
    "hypotenuse": 5,
    "log_a": 1.099,
    "clamped": 7
  },
  "kv": {
    "greeting": "Hello from the VM!",
    "keys": ["hypotenuse", "greeting"]
  },
  "time": {
    "elapsed_ms": 0,
    "timestamp": 1774515937
  }
}
```

Pass custom JavaScript and event data as arguments:

```bash
cd js-host && cargo run --release -- \
  'import * as math from "math";
   function handler(event) { return { result: math.sqrt(event.x) }; }
   export { handler };' \
  '{"x": 144}'
```

#### Host Plugins

Plugins are Rust functions registered as host modules on the sandbox **before** the JS runtime loads. Guest JavaScript imports them as ES modules (e.g. `import * as math from "math"`). This follows the same pattern used in [hyperlight](https://github.com/hyperlight-dev/hyperlight) and [hyperagent](https://github.com/hyperlight-dev/hyperagent).

| Plugin | Module | Functions |
|--------|--------|-----------|
| **math** | `"math"` | `sqrt`, `pow`, `abs`, `floor`, `ceil`, `round`, `log`, `min`, `max` |
| **time** | `"time"` | `now_ms` (epoch millis), `now_secs` (epoch seconds) |
| **kv** | `"kv"` | `set`, `get`, `delete`, `keys` (in-memory key-value store) |

To add a new plugin, create a struct implementing the `Plugin` trait in `js-host/src/plugins/` and register it in `all_plugins()`. No changes to `main.rs` needed.

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
