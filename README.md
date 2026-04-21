# saferun

⚠️ Running random scripts? Don’t trust them blindly.

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
- access parts of your system you dind't expect

## The solution
```saferun``` provides a safer way to run scripts locally
- runs script in a temporary directory
- clears the environment
- limits available tools
- blocks obvious network access

## Quick start

### Install

### Download binary
```bash
wget https://github.com/pduarte21/saferun/releases/download/v0.1.0/saferun-linux-x86_64

chmod +x saferun-linux-x86_64
mv saferun-linux-x86_64 saferun
```

### Run
```
./saferun run script.sh
```

## What saferun does
When you run a script with `saferun`:
- it executes in an isolated temporary filesystem
- it cannot access your environment variables
- it runs with a minimal PATH
- basic network tools are blocked

## Examples
Try the included scripts:
```
./saferun run examples/hello.sh
./saferun run examples/env_leak.sh
./saferun run examples/evil_write.sh
./saferun run examples/network_attempt.sh
./saferun run examples/harmless_but_complex.sh
```

## What saferun protect againts
- accidental execution of unsafe scripts
- leaking environment variables
- basic filesystem side-effect
- obvious network calls (e.g. curl, wget)

## Limitations
This is an early version and **not a full security sandbox**.

It does NOT yet prevent:
- access to files via absolute paths
- advanced or obfuscated commands
- indirect execution patterns
- real network isolation

## Why not just use a VM
VMs provide stronger isolation, but:
- they add friction
- they are not integrated into everyday workflows

`saferun` is designed to be:
> a lightweight, low friction safer default

## Status
Early prototype, focused on simplicity and usability.