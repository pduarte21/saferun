# saferun

⚠️ Stop blindly running scripts.

**Preview what a shell script will do before running it.**

Run untrusted scripts safely — no setup, no surprises.

---

## The problem
We've all done this:
```bash
curl something.sh | bash
```

You don't really know what that script will do.

It might:
- modify your files
- leak environment variables
- access unexpected parts of your system
- execute other scripts

## The solution
```saferun``` helps you **understand and control** what a script does before you run it.

### Preview mode (dry-run)
```
saferun run script.sh --dry-run
```

Example output:
```
[saferun] Dry run mode (no execution)

[info] Script summary:
 - commands: 8

[warning] Potentially dangerous patterns:
 - rm -rf
 - curl
 - piped execution

[info] Network access: detected

[info] File operations detected:
 - rm -rf /tmp/*
 - echo "data" > file.txt

[saferun] No changes were made.
```

### Safety Prompt
If a script looks suspicious, `saferun` asks before running:
```
[saferun] ⚠️  Potentially dangerous patterns detected:
 - rm -rf
 - curl

Continue? (y/N):
```

### Lightweight isolation
When you do run a script:
```
saferun run script.sh
```
It runs with basic protections:
- temporary working directory
- clean environment (`env_clear`)
- minimal `PATH`
- isolated `HOME`
- basic blocking of network tools

## Install

### Download binary
Go to the [Releases page](https://github.com/pduarte21/saferun/releases) and download the binary for your OS.

#### Linux (x86_64)
```bash
wget https://github.com/pduarte21/saferun/releases/download/v0.1.0/saferun-linux-x86_64
chmod +x saferun-linux-x86_64
mv saferun-linux-x86_64 saferun
```

#### macOS (Apple Silicon)
```bash
wget https://github.com/pduarte21/saferun/releases/download/v0.1.0/saferun-macos-aarch64
chmod +x saferun-macos-aarch64
mv saferun-macos-aarch64 saferun
```

#### Windows
Download `saferun-windows-x86_64.exe` and run it from PowerShell or CMD.

### Run
```
./saferun run script.sh
```

## Examples
Try the included scripts:
```
./saferun run examples/hello.sh
./saferun run examples/env_leak.sh
./saferun run examples/evil_write.sh
./saferun run examples/network_attempt.sh
./saferun run examples/harmless_but_complex.sh
./saferun run examples/suspicious-install.sh --dry-run
./saferun run examples/suspicious-install.sh
```

## What saferun protect againts
- blindly running unknown scripts
- accidental file modifications
- leaking environment variables
- obvious unsafe patterns (e.g. `rm -rf`, `curl`, `chmod 777`)
- basic network calls

## Limitations
This is **not a full sandbox**.

It does NOT prevent:
- access via absolute paths
- dynamic or obfuscated behaviour
- indirect execution
- real OS-level isolation

## Why not just use a VM
VMs and container provide stronger isolation, but:
- they add friction
- they are not used for quick, everyday scripts

`saferun` is designed to be:
> a fast, low-friction safety layer for the common case

## Status
Early prototype focused on:
- usability
- visibility
- real-world developer workflows

Feedback is very welcome.